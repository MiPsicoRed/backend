pub mod user;
pub mod user_token;

use std::sync::Arc;

use crate::{
    adapters::{crypto::jwt::Claims, http::app_state::AppState},
    app_error::AppError,
    use_cases::user::UserJwtService,
};
use axum::{Extension, Router, extract::Request, middleware::Next, response::Response};

/// Trait that a Payload should implement in order to be validated (TODO: Can I enforce this)
trait Validateable {
    fn valid(&self) -> bool;
}

/// Represents an authenticated user, that we get from the auth middleware and can be used in the requests
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
    pub name: String,
    pub role_id: i32,
    pub verified: bool,
}

impl From<Claims> for AuthUser {
    fn from(value: Claims) -> Self {
        Self {
            user_id: value.uuid,
            name: value.name,
            role_id: value.role,
            verified: value.verified,
        }
    }
}

/// Middleware that extracts the bearer Token from the request and verifies it.
async fn auth_middleware(
    Extension(user_jwt_service): Extension<Arc<dyn UserJwtService>>,
    mut request: Request,
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
    let claims = user_jwt_service
        .validate_token(token)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

    // Insert the authenticated user into request extensions
    // we should be able to extract AuthUser now from endpoints using: Extension(auth_user): Extension<AuthUser> as a param in our endpoint
    request.extensions_mut().insert(AuthUser::from(claims));

    Ok(next.run(request).await)
}

/// Define the routes of out backend
pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/user", user::router())
        .nest("/user_token", user_token::router())
}
