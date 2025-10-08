use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info};
use utoipa::ToSchema;

use crate::{
    app_error::{AppResult},
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ValidatePayload {
}


#[derive(Debug, Serialize, ToSchema)]
pub struct ValidateResponse {
    valid: bool,
}

#[utoipa::path(
    post, 
    path = "/api/user_token/validate",
    responses(
        (status = 200, description = "Token is valid", body = ValidateResponse),
        (status = 401, description = "Token is invalid or expired"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "User Token",
    summary = "Validates a JWT token to see if  it's valid"
)]
pub async fn validate_token(
) -> AppResult<impl IntoResponse> {
    info!("Validate user token called");

    Ok((
        StatusCode::OK,
        Json(ValidateResponse {
            valid: true
        }),
    ))
}
