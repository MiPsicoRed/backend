use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{AuthUser, Validateable}, app_error::{AppError, AppResult}, use_cases::user::UserUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct CheckoutQuery {
    #[param(example = "insert-product-id")]
    product_id: String,
}

impl Validateable for CheckoutQuery {
    fn valid(&self) -> bool {
        !self.product_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CheckoutResponse {
    checkout_url: String,
    success: bool,
}

#[utoipa::path(get, path = "/api/user/checkout", 
    params(CheckoutQuery),
    responses( 
        (status = 200, description = "Checkout URL created", body = CheckoutResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "User",
    summary = "Create checkout URL for product purchase",
    description = "\n\n**Required:**  Verified Email & Requesting User Id"
)]
#[instrument(skip(use_cases))]
pub async fn create_checkout(
    Extension(auth_user): Extension<AuthUser>,
    State(use_cases): State<Arc<UserUseCases>>,
    Query(params): Query<CheckoutQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Create checkout called");

    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let user_uuid = Uuid::parse_str(&auth_user.user_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;
    let success_url = "https://mipsicored.com/purchase/success"; // TODO: Get from config

    let checkout_url = use_cases
        .create_purchase_checkout(&user_uuid, &params.product_id, success_url)
        .await?;

    Ok((
        StatusCode::OK,
        Json(CheckoutResponse {
            success: true,
            checkout_url,
        }),
    ))
}