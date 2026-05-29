use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::{app_error::AppResult, entities::message::Message};

#[async_trait]
pub trait MessagePersistence: Send + Sync {
    async fn create(&self, message: &Message) -> AppResult<()>;
    async fn read_conversation(&self, user1_id: &Uuid, user2_id: &Uuid) -> AppResult<Vec<Message>>;
    async fn mark_as_read(&self, message_id: &Uuid) -> AppResult<()>;
}

#[derive(Clone)]
pub struct MessageUseCases {
    persistence: Arc<dyn MessagePersistence>,
}

impl MessageUseCases {
    pub fn new(persistence: Arc<dyn MessagePersistence>) -> Self {
        Self { persistence }
    }

    pub async fn create(&self, message: &Message) -> AppResult<()> {
        self.persistence.create(message).await
    }

    pub async fn read_conversation(&self, user1_id: &Uuid, user2_id: &Uuid) -> AppResult<Vec<Message>> {
        self.persistence.read_conversation(user1_id, user2_id).await
    }

    pub async fn mark_as_read(&self, message_id: &Uuid) -> AppResult<()> {
        self.persistence.mark_as_read(message_id).await
    }
}
