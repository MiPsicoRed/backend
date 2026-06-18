use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::{app_error::AppResult, entities::notification::Notification};

#[async_trait]
pub trait NotificationPersistence: Send + Sync {
    async fn create(&self, notification: &Notification) -> AppResult<()>;
    async fn read_for_user(&self, user_id: &Uuid) -> AppResult<Vec<Notification>>;
    async fn mark_as_read(&self, notification_id: &Uuid) -> AppResult<()>;
    async fn mark_all_as_read(&self, user_id: &Uuid) -> AppResult<()>;
}

#[derive(Clone)]
pub struct NotificationUseCases {
    persistence: Arc<dyn NotificationPersistence>,
}

impl NotificationUseCases {
    pub fn new(persistence: Arc<dyn NotificationPersistence>) -> Self {
        Self { persistence }
    }

    pub async fn create(&self, notification: &Notification) -> AppResult<()> {
        self.persistence.create(notification).await
    }

    pub async fn read_for_user(&self, user_id: &Uuid) -> AppResult<Vec<Notification>> {
        self.persistence.read_for_user(user_id).await
    }

    pub async fn mark_as_read(&self, notification_id: &Uuid) -> AppResult<()> {
        self.persistence.mark_as_read(notification_id).await
    }
    
    pub async fn mark_all_as_read(&self, user_id: &Uuid) -> AppResult<()> {
        self.persistence.mark_all_as_read(user_id).await
    }
}
