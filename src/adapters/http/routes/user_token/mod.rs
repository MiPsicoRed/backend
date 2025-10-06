use axum::routing::get;
use axum::{Router, middleware, routing::post};

use crate::adapters::http::routes::user_token::generate::generate_token;
use crate::adapters::http::routes::user_token::verify::verify;
use crate::adapters::http::{app_state::AppState, routes::auth_middleware};

mod generate;
mod verify;

pub fn router() -> Router<AppState> {
    let public_routes = Router::new().route("/verify", get(verify));

    let protected_routes = Router::new()
        .route("/generate", post(generate_token))
        .layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}
