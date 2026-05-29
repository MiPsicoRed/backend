use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::message::Message,
    use_cases::message::MessagePersistence,
};

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct MessageDb {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub content: String,
    pub read_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<MessageDb> for Message {
    fn from(db: MessageDb) -> Self {
        Message {
            id: db.id,
            sender_id: db.sender_id,
            receiver_id: db.receiver_id,
            content: db.content,
            read_at: db.read_at,
            created_at: db.created_at,
        }
    }
}

#[async_trait]
impl MessagePersistence for PostgresPersistence {
    async fn create(&self, message: &Message) -> AppResult<()> {
        sqlx::query!(
            "INSERT INTO messages (id, sender_id, receiver_id, content) 
             VALUES ($1, $2, $3, $4)",
            message.id,
            message.sender_id,
            message.receiver_id,
            message.content
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    async fn read_conversation(&self, user1_id: &Uuid, user2_id: &Uuid) -> AppResult<Vec<Message>> {
        sqlx::query_as!(
            MessageDb,
            r#"
                SELECT id, sender_id, receiver_id, content, read_at, created_at
                FROM messages
                WHERE (sender_id = $1 AND receiver_id = $2) OR (sender_id = $2 AND receiver_id = $1)
                ORDER BY created_at ASC
            "#,
            user1_id,
            user2_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|msgs| msgs.into_iter().map(Message::from).collect())
    }

    async fn mark_as_read(&self, message_id: &Uuid) -> AppResult<()> {
        sqlx::query!(
            "UPDATE messages SET read_at = CURRENT_TIMESTAMP WHERE id = $1",
            message_id
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }
}
