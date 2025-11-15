use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Serialize};
use tracing::{info, warn};
use utoipa::ToSchema;

use crate::{
    app_error::{AppError, AppResult}, use_cases::user_token::UserTokenUseCases,
};

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
    State(user_token_use_cases): State<Arc<UserTokenUseCases>>,
    headers: axum::http::HeaderMap,
) -> AppResult<impl IntoResponse> {
    info!("Validate user token called");

    let auth_header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| {
            AppError::Unauthorized("Missing Authorization header".to_string())
        })?;
    
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| {
            AppError::Unauthorized("Invalid Authorization header format".to_string())
        })?;
    
    user_token_use_cases
        .validate_token(token)
        .await
        .map_err(|e| {
            warn!("Token validation failed: {:?}", e);
            AppError::Unauthorized("Invalid or expired token".to_string())
        })?;
    
    Ok((
        StatusCode::OK,
        Json(ValidateResponse { valid: true }),
    ))
}
