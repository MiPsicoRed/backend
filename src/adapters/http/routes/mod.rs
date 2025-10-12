pub mod patient;
pub mod professional;
pub mod professional_language;
pub mod professional_specialization;
pub mod session;
pub mod session_type;
pub mod user;
pub mod user_token;

use std::sync::Arc;

use crate::{
    adapters::{crypto::jwt::Claims, http::app_state::AppState},
    app_error::AppError,
    entities::user::Role,
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

    // Verify token and get user
    let claims = user_jwt_service
        .validate_token(token)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

    // Insert the authenticated user into request extensions
    // we should be able to extract AuthUser now from endpoints using: Extension(auth_user): Extension<AuthUser> as a param in our endpoint
    request.extensions_mut().insert(AuthUser::from(claims));

    Ok(next.run(request).await)
}

/// Middleware that only let's a request through if the user claims to be verified in the jwt
async fn verified_middleware(
    Extension(auth_user): Extension<AuthUser>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    if !auth_user.verified {
        return Err(AppError::Unauthorized("User not verified".to_string()));
    }
    Ok(next.run(request).await)
}

#[derive(Clone)]
struct RequiredRoles(Vec<Role>);

async fn require_role_middleware(
    Extension(auth_user): Extension<AuthUser>,
    Extension(required_roles): Extension<RequiredRoles>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let user_role = Role::from_id(auth_user.role_id).unwrap_or(Role::Patient);

    if !required_roles.0.contains(&user_role) {
        return Err(AppError::Unauthorized(
            "Insufficient permissions".to_string(),
        ));
    }

    Ok(next.run(request).await)
}

/// Helper functions for common roles
#[allow(dead_code)]
fn require_roles(roles: Vec<Role>) -> Extension<RequiredRoles> {
    Extension(RequiredRoles(roles))
}

#[allow(dead_code)]
fn require_admin() -> Extension<RequiredRoles> {
    Extension(RequiredRoles(vec![Role::Admin]))
}

#[allow(dead_code)]
fn require_professional_or_admin() -> Extension<RequiredRoles> {
    Extension(RequiredRoles(vec![Role::Admin, Role::Professional]))
}

#[allow(dead_code)]
fn require_patient_or_admin() -> Extension<RequiredRoles> {
    Extension(RequiredRoles(vec![Role::Admin, Role::Patient]))
}

/// Define the routes of out backend
pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/user", user::router())
        .nest("/user_token", user_token::router())
        .nest("/patient", patient::router())
        .nest("/session_type", session_type::router())
        .nest("/session", session::router())
        .nest("/professional", professional::router())
        .nest("/professional_language", professional_language::router())
        .nest(
            "/professional_specialization",
            professional_specialization::router(),
        )
}
