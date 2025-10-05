use axum::{Router, middleware, routing::post};

use crate::adapters::http::routes::user_token::generate::generate_token;
use crate::adapters::http::{app_state::AppState, routes::auth_middleware};

mod generate;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/generate", post(generate_token))
        .layer(middleware::from_fn(auth_middleware))
}
