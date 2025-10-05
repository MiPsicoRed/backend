use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::adapters::http::routes::user::get_all::get_all_users;
use crate::adapters::http::routes::user::login::login;
use crate::adapters::http::routes::user::register::register;
use crate::adapters::http::{app_state::AppState, routes::auth_middleware};

mod get_all;
mod login;
mod register;

pub fn router() -> Router<AppState> {
    let public_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));

    let protected_routes = Router::new()
        .route("/all", get(get_all_users))
        .layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}
