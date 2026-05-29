use axum::{Extension, Json, Router, extract::{Path, State}, middleware, routing::{get, post}};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    adapters::http::{app_state::AppState, routes::{AuthUser, auth_middleware, verified_middleware}},
    app_error::{AppError, AppResult},
    entities::message::Message,
};

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub receiver_id: Uuid,
    pub content: String,
}

pub async fn send_message(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<SendMessageRequest>,
) -> AppResult<Json<Message>> {
    let sender_id = Uuid::parse_str(&auth_user.user_id).unwrap();

    let message = Message {
        id: Uuid::new_v4(),
        sender_id,
        receiver_id: payload.receiver_id,
        content: payload.content,
        read_at: None,
        created_at: None,
    };

    state.message_use_cases.create(&message).await?;
    Ok(Json(message))
}

pub async fn get_conversation(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(other_user_id): Path<Uuid>,
) -> AppResult<Json<Vec<Message>>> {
    let user_id = Uuid::parse_str(&auth_user.user_id).unwrap();
    let msgs = state.message_use_cases.read_conversation(&user_id, &other_user_id).await?;
    Ok(Json(msgs))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/send", post(send_message))
        .route("/conversation/:other_user_id", get(get_conversation))
        .layer(middleware::from_fn(verified_middleware))
        .layer(middleware::from_fn(auth_middleware))
}
