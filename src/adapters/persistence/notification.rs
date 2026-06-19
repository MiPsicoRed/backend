use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::notification::Notification,
    use_cases::notification::NotificationPersistence,
};

#[async_trait]
impl NotificationPersistence for PostgresPersistence {
    async fn create(&self, notification: &Notification) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO notifications (id, user_id, type, title, message, is_read, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(notification.id)
        .bind(notification.user_id)
        .bind(notification.r#type.clone())
        .bind(notification.title.clone())
        .bind(notification.message.clone())
        .bind(notification.is_read)
        .bind(notification.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error creating notification: {:?}", e);
            AppError::Database(e)
        })?;

        Ok(())
    }

    async fn read_for_user(&self, user_id: &Uuid) -> AppResult<Vec<Notification>> {
        let notifications = sqlx::query_as::<_, Notification>(
            r#"
            SELECT id, user_id, type, title, message, is_read, created_at
            FROM notifications
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 50
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error reading notifications: {:?}", e);
            AppError::Database(e)
        })?;

        Ok(notifications)
    }

    async fn mark_as_read(&self, notification_id: &Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE notifications
            SET is_read = true
            WHERE id = $1
            "#
        )
        .bind(notification_id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error marking notification as read: {:?}", e);
            AppError::Database(e)
        })?;

        Ok(())
    }

    async fn mark_all_as_read(&self, user_id: &Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE notifications
            SET is_read = true
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error marking all notifications as read: {:?}", e);
            AppError::Database(e)
        })?;

        Ok(())
    }
}
