use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Serialize};
use tracing::{info, instrument};
use utoipa::{ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{AuthUser}, app_error::{AppError, AppResult}, use_cases::user::UserUseCases
};


#[derive(Debug, Serialize, ToSchema)]
pub struct PortalResponse {
    portal_url: String,
    success: bool,
}

#[utoipa::path(get, path = "/api/user/portal", 
    responses( 
        (status = 200, description = "Customer portal URL", body = PortalResponse),
        (status = 404, description = "Customer not found - no purchases yet"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "User",
    summary = "Get customer portal URL to view invoices and purchases",
    description = "\n\n**Required:**  Verified Email"
)]
#[instrument(skip(use_cases))]
pub async fn get_portal_url(
    Extension(auth_user): Extension<AuthUser>,
    State(use_cases): State<Arc<UserUseCases>>,
) -> AppResult<impl IntoResponse> {
    info!("Get portal URL called");

    let user_uuid = Uuid::parse_str(&auth_user.user_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let portal_url = use_cases
        .get_customer_portal_url(&user_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(PortalResponse {
            success: true,
            portal_url,
        }),
    ))
}