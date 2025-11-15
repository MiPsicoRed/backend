use axum::{
    Router, middleware,
    routing::{get, post},
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::adapters::http::routes::{
    require_admin, require_role_middleware,
    user::{onboard::onboard_user, register::register},
};
use crate::adapters::http::routes::{user::login::login, verified_middleware};
use crate::adapters::http::{app_state::AppState, routes::auth_middleware};
use crate::{adapters::http::routes::user::get_all::get_all_users, entities::user::User};

pub mod get_all;
pub mod login;
pub mod onboard;
pub mod register;

#[derive(Debug, Serialize, ToSchema)]
struct UserResponse {
    pub id: Uuid,
    pub role_id: i32, // should we return the string or the id
    pub username: String,
    pub usersurname: String,
    pub email: String,
    pub verified: Option<bool>,
    pub needs_onboarding: Option<bool>,
    pub password_hash: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            role_id: user.role.to_id(),
            username: user.username,
            usersurname: user.usersurname,
            email: user.email,
            verified: user.verified,
            needs_onboarding: user.needs_onboarding,
            password_hash: String::new(), // just in case, never map the password hash to a response
            created_at: user.created_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    let public_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));

    let protected_routes = Router::new()
        .route(
            "/all",
            get(get_all_users)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_admin()), // Extension needs to go AFTER the middleware
        )
        .route("/onboarded", post(onboard_user))
        .layer(middleware::from_fn(verified_middleware))
        .layer(middleware::from_fn(auth_middleware)); // Main auth middleware always has to be the LAST

    Router::new().merge(public_routes).merge(protected_routes)
}
