use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::session_type::SessionType,
    use_cases::session_type::SessionTypePersistence,
};

// Patient struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct SessionTypeDb {
    pub id: Uuid,
    pub session_type_name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<SessionTypeDb> for SessionType {
    fn from(session_type_db: SessionTypeDb) -> Self {
        SessionType {
            id: session_type_db.id,
            name: session_type_db.session_type_name,
            created_at: session_type_db.created_at,
        }
    }
}

#[async_trait]
impl SessionTypePersistence for PostgresPersistence {
    async fn create(&self, name: &str) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO session_types (id, session_type_name) 
                    VALUES ($1, $2)",
            uuid,
            name
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn read_all(&self) -> AppResult<Vec<SessionType>> {
        sqlx::query_as!(
            SessionTypeDb,
            r#"
                SELECT id, session_type_name, created_at
                FROM session_types
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|res| res.into_iter().map(SessionType::from).collect())
    }

    async fn read_single(&self, id: Uuid) -> AppResult<SessionType> {
        sqlx::query_as!(
            SessionTypeDb,
            r#"
                SELECT id, session_type_name, created_at
                FROM session_types 
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(SessionType::from)
    }

    async fn update(&self, id: Uuid, name: &str) -> AppResult<()> {
        sqlx::query!(
            "UPDATE session_types 
                SET session_type_name = $2
                WHERE id = $1",
            id,
            name
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM session_types WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }
}
