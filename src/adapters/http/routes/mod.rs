pub mod user;

use std::sync::Arc;

use crate::{
    adapters::http::app_state::AppState, app_error::AppError, use_cases::user::UserJwtService,
};
use axum::{Extension, Router, extract::Request, middleware::Next, response::Response};

/// Trait that a Payload should implement in order to be validated (TODO: Can I enforce this)
trait Validateable {
    fn valid(&self) -> bool;
}

/// Middleware that extracts the bearer Token from the request and verifies it.
async fn auth_middleware(
    Extension(user_jwt_service): Extension<Arc<dyn UserJwtService>>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid authorization format".to_string()))?;

    // Verify token and get user (TODO: Get claims and insert into request extensions)
    user_jwt_service
        .validate_token(token)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

    // Insert the authenticated user into request extensions
    // request.extensions_mut().insert(AuthUser {
    //     user_id: user.id,
    //     username: user.username,
    // });

    Ok(next.run(request).await)
}

/// Define the routes of out backend
pub fn router() -> Router<AppState> {
    Router::new().nest("/user", user::router())
}
