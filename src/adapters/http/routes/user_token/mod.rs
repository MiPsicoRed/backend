use axum::routing::get;
use axum::{Router, middleware, routing::post};
use serde::Serialize;
use uuid::Uuid;

use crate::adapters::http::routes::user_token::generate::generate_token;
use crate::adapters::http::routes::user_token::verify::verify;
use crate::adapters::http::{app_state::AppState, routes::auth_middleware};
use crate::entities::user_token::UserToken;

mod generate;
mod verify;

#[derive(Debug, Serialize)]
struct UserTokenResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<UserToken> for UserTokenResponse {
    fn from(user_token: UserToken) -> Self {
        UserTokenResponse {
            id: user_token.id,
            user_id: user_token.user_id,
            token: user_token.token,
            expires_at: user_token.expires_at,
            created_at: user_token.created_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    let public_routes = Router::new().route("/verify", get(verify));

    let protected_routes = Router::new()
        .route("/generate", post(generate_token))
        .layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}
