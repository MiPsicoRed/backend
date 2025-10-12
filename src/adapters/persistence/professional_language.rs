use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::professional_language::ProfessionalLanguage,
    use_cases::professional_language::ProfessionalLanguagePersistence,
};

// Professional struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ProfessionalLanguageDb {
    pub id: Uuid,
    pub professional_id: Uuid,
    pub name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<ProfessionalLanguageDb> for ProfessionalLanguage {
    fn from(professional_db: ProfessionalLanguageDb) -> Self {
        ProfessionalLanguage {
            id: Some(professional_db.id),
            professional_id: Some(professional_db.professional_id),
            name: professional_db.name,
            created_at: professional_db.created_at,
        }
    }
}

#[async_trait]
impl ProfessionalLanguagePersistence for PostgresPersistence {
    async fn create(&self, professional_language: &ProfessionalLanguage) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO professionals_languages (id, professional_id, p_language) 
                    VALUES ($1, $2, $3)",
            uuid,
            professional_language.professional_id,
            professional_language.name
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn read_all(&self) -> AppResult<Vec<ProfessionalLanguage>> {
        sqlx::query_as!(
            ProfessionalLanguageDb,
            r#"
                SELECT id, professional_id, p_language as name, created_at
                FROM professionals_languages
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|languages| {
            languages
                .into_iter()
                .map(ProfessionalLanguage::from)
                .collect()
        })
    }

    async fn read_single(&self, id: Uuid) -> AppResult<ProfessionalLanguage> {
        sqlx::query_as!(
            ProfessionalLanguageDb,
            r#"
                SELECT id, professional_id, p_language as name, created_at
                FROM professionals_languages 
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(ProfessionalLanguage::from)
    }

    async fn update(&self, professional_language: &ProfessionalLanguage) -> AppResult<()> {
        sqlx::query!(
            "UPDATE professionals_languages 
                SET p_language = $2
                WHERE id = $1",
            professional_language.id,
            professional_language.name
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM professionals_languages WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }
}
