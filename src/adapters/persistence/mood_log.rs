use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::mood_log::MoodLog,
    use_cases::mood_log::MoodLogPersistence,
};

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct MoodLogDb {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub mood_score: i32,
    pub note: Option<String>,
    pub shared_with_professional: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<MoodLogDb> for MoodLog {
    fn from(db: MoodLogDb) -> Self {
        MoodLog {
            id: db.id,
            patient_id: db.patient_id,
            mood_score: db.mood_score,
            note: db.note,
            shared_with_professional: db.shared_with_professional,
            created_at: db.created_at,
        }
    }
}

#[async_trait]
impl MoodLogPersistence for PostgresPersistence {
    async fn create(&self, mood_log: &MoodLog) -> AppResult<()> {
        sqlx::query!(
            "INSERT INTO mood_logs (id, patient_id, mood_score, note, shared_with_professional) 
             VALUES ($1, $2, $3, $4, $5)",
            mood_log.id,
            mood_log.patient_id,
            mood_log.mood_score,
            mood_log.note,
            mood_log.shared_with_professional
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    async fn read_by_patient(&self, patient_id: &Uuid) -> AppResult<Vec<MoodLog>> {
        sqlx::query_as!(
            MoodLogDb,
            r#"
                SELECT id, patient_id, mood_score, note, shared_with_professional, created_at
                FROM mood_logs
                WHERE patient_id = $1
                ORDER BY created_at DESC
            "#,
            patient_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|logs| logs.into_iter().map(MoodLog::from).collect())
    }

    async fn read_shared_by_patient(&self, patient_id: &Uuid) -> AppResult<Vec<MoodLog>> {
        sqlx::query_as!(
            MoodLogDb,
            r#"
                SELECT id, patient_id, mood_score, note, shared_with_professional, created_at
                FROM mood_logs
                WHERE patient_id = $1 AND shared_with_professional = TRUE
                ORDER BY created_at DESC
            "#,
            patient_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|logs| logs.into_iter().map(MoodLog::from).collect())
    }

    async fn delete(&self, id: &Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM mood_logs WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }
}
