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
pub struct CheckAccessQuery {
    #[param(example = "insert-product-id")]
    product_id: String,
}

impl Validateable for CheckAccessQuery {
    fn valid(&self) -> bool {
        !self.product_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CheckAccessResponse {
    has_access: bool,
    success: bool,
}

#[utoipa::path(get, path = "/api/user/purchases/access", 
    params(CheckAccessQuery),
    responses( 
        (status = 200, description = "Access check result", body = CheckAccessResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "User",
    summary = "Check if user has access to a product",
    description = "\n\n**Required:**  Verified Email"
)]
#[instrument(skip(use_cases))]
pub async fn check_access(
    Extension(auth_user): Extension<AuthUser>,
    State(use_cases): State<Arc<UserUseCases>>,
    Query(params): Query<CheckAccessQuery>,
) -> AppResult<impl IntoResponse> {
     info!("Create checkout called");

    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let user_uuid = Uuid::parse_str(&auth_user.user_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let has_access = use_cases
        .check_product_access(&user_uuid, &params.product_id)
        .await?;

    Ok((
        StatusCode::OK,
        Json(CheckAccessResponse {
            success: true,
            has_access,
        }),
    ))
}