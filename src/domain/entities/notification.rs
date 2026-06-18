use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub r#type: String, // 'info' | 'success' | 'warning' | 'default'
    pub title: String,
    pub message: String,
    pub is_read: bool,
    pub created_at: Option<DateTime<Utc>>,
}
