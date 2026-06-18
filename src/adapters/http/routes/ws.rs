use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade}, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    adapters::http::app_state::AppState,
    infra::websocket::WebSocketManager,
};

#[derive(Deserialize)]
pub struct WsParams {
    token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(params): Query<WsParams>,
) -> impl IntoResponse {
    // Validate token
    let claims = match state.user_use_cases.jwt_service.validate_token(&params.token) {
        Ok(claims) => claims,
        Err(_) => return axum::http::StatusCode::UNAUTHORIZED.into_response(),
    };

    let user_id = match Uuid::parse_str(&claims.uuid) {
        Ok(id) => id,
        Err(_) => return axum::http::StatusCode::UNAUTHORIZED.into_response(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, user_id, state.websocket_manager.clone()))
}

async fn handle_socket(mut socket: WebSocket, user_id: Uuid, ws_manager: Arc<WebSocketManager>) {
    let (tx, mut rx) = mpsc::channel(100);

    // Register connection
    ws_manager.add_connection(user_id, tx).await;

    // We only need to send messages TO the client for now, we don't expect messages FROM the client.
    // So we just spawn a task to forward messages from the channel to the socket.
    
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if socket.send(msg).await.is_err() {
                break; // client disconnected
            }
        }
    });

    // Wait until the socket closes or errors
    let _ = recv_task.await;

    // Unregister connection
    ws_manager.remove_connection(user_id).await;
}

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().route("/", axum::routing::get(ws_handler))
}
