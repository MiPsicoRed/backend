use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::session::{Session, SessionStatus},
    use_cases::session::SessionPersistence,
};

// Session struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct SessionDb {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub professional_id: Uuid,
    pub session_type_id: Option<Uuid>,
    pub session_status_id: i32,
    pub session_date: Option<chrono::NaiveDateTime>,
    pub videocall_url: Option<String>,
    pub notes: Option<String>,
    pub completed: Option<bool>,
    pub session_duration: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<SessionDb> for Session {
    fn from(session_db: SessionDb) -> Self {
        Session {
            id: Some(session_db.id),
            patient_id: session_db.patient_id,
            professional_id: session_db.professional_id,
            session_type_id: session_db.session_type_id,
            session_status: SessionStatus::from_id(session_db.session_status_id)
                .unwrap_or_default(),
            session_date: session_db.session_date,
            videocall_url: session_db.videocall_url,
            notes: session_db.notes,
            completed: session_db.completed.unwrap_or(false),
            session_duration: session_db.session_duration,
            created_at: session_db.created_at,
        }
    }
}

#[async_trait]
impl SessionPersistence for PostgresPersistence {
    async fn create(&self, session: &Session) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO sessions (id, patient_id, professional_id, session_type_id, session_status_id, session_date, videocall_url, notes, completed, session_duration) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            uuid,
            session.patient_id,
            session.professional_id,
            session.session_type_id,
            session.session_status.to_id(),
            session.session_date,
            session.videocall_url,
            session.notes,
            session.completed,
            session.session_duration
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn read_all(&self) -> AppResult<Vec<Session>> {
        sqlx::query_as!(
            SessionDb,
            r#"
                SELECT id, patient_id, professional_id, session_type_id, session_status_id, session_date, videocall_url, notes, completed, session_duration, created_at
                FROM sessions
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|sessions| sessions.into_iter().map(Session::from).collect())
    }

    async fn read_single(&self, id: Uuid) -> AppResult<Session> {
        sqlx::query_as!(
            SessionDb,
            r#"
                SELECT id, patient_id, professional_id, session_type_id, session_status_id, session_date, videocall_url, notes, completed, session_duration, created_at
                FROM sessions 
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(Session::from)
    }

    async fn update(&self, session: &Session) -> AppResult<()> {
        sqlx::query!(
            "UPDATE sessions 
                SET patient_id = $2, professional_id = $3, session_type_id = $4, session_status_id = $5, session_date = $6, videocall_url = $7, notes = $8, completed = $9, session_duration = $10
                WHERE id = $1",
            session.id,
            session.patient_id,
            session.professional_id,
            session.session_type_id,
            session.session_status.to_id(),
            session.session_date,
            session.videocall_url,
            session.notes,
            session.completed,
            session.session_duration
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM sessions WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }
}
