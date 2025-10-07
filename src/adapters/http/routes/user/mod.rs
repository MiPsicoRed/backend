use axum::{
    Router, middleware,
    routing::{get, post},
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::adapters::http::routes::user::login::login;
use crate::adapters::http::routes::user::register::register;
use crate::adapters::http::{app_state::AppState, routes::auth_middleware};
use crate::{adapters::http::routes::user::get_all::get_all_users, entities::user::User};

pub mod get_all;
pub mod login;
pub mod register;

#[derive(Debug, Serialize, ToSchema)]
struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub usersurname: String,
    pub email: String,
    pub phone: String,
    pub birthdate: Option<chrono::NaiveDate>,
    pub verified: Option<bool>,
    pub password_hash: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            usersurname: user.usersurname,
            email: user.email,
            phone: user.phone,
            birthdate: user.birthdate,
            verified: user.verified,
            password_hash: user.password_hash,
            created_at: user.created_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    let public_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));

    let protected_routes = Router::new()
        .route("/all", get(get_all_users))
        .layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}
