use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use axum::extract::ws::Message;

#[derive(Clone)]
pub struct WebSocketManager {
    // Map of user_id -> channel sender. 
    // mpsc is used so that multiple tasks can queue messages to be sent down the WS.
    connections: Arc<RwLock<HashMap<Uuid, mpsc::Sender<Message>>>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_connection(&self, user_id: Uuid, tx: mpsc::Sender<Message>) {
        let mut conns = self.connections.write().await;
        conns.insert(user_id, tx);
        tracing::debug!("WebSocket connected for user: {}", user_id);
    }

    pub async fn remove_connection(&self, user_id: Uuid) {
        let mut conns = self.connections.write().await;
        conns.remove(&user_id);
        tracing::debug!("WebSocket disconnected for user: {}", user_id);
    }

    pub async fn send_message(&self, user_id: Uuid, message: String) -> bool {
        let conns = self.connections.read().await;
        if let Some(tx) = conns.get(&user_id) {
            if let Err(e) = tx.send(Message::Text(message.into())).await {
                tracing::warn!("Failed to send WS message to user {}: {:?}", user_id, e);
                return false;
            }
            return true;
        }
        false
    }
}
