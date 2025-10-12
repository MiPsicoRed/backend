use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::professional_specialization::ProfessionalSpecialization,
    use_cases::professional_specialization::ProfessionalSpecializationPersistence,
};

// Professional struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ProfessionalSpecializationDb {
    pub id: Uuid,
    pub professional_id: Uuid,
    pub name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<ProfessionalSpecializationDb> for ProfessionalSpecialization {
    fn from(professional_db: ProfessionalSpecializationDb) -> Self {
        ProfessionalSpecialization {
            id: Some(professional_db.id),
            professional_id: Some(professional_db.professional_id),
            name: professional_db.name,
            created_at: professional_db.created_at,
        }
    }
}

#[async_trait]
impl ProfessionalSpecializationPersistence for PostgresPersistence {
    async fn create(
        &self,
        professional_specialization: &ProfessionalSpecialization,
    ) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO professionals_specializations (id, professional_id, s_name) 
                    VALUES ($1, $2, $3)",
            uuid,
            professional_specialization.professional_id,
            professional_specialization.name
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn read_all(&self) -> AppResult<Vec<ProfessionalSpecialization>> {
        sqlx::query_as!(
            ProfessionalSpecializationDb,
            r#"
                SELECT id, professional_id, s_name as name, created_at
                FROM professionals_specializations
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|languages| {
            languages
                .into_iter()
                .map(ProfessionalSpecialization::from)
                .collect()
        })
    }

    async fn read_single(&self, id: Uuid) -> AppResult<ProfessionalSpecialization> {
        sqlx::query_as!(
            ProfessionalSpecializationDb,
            r#"
                SELECT id, professional_id, s_name as name, created_at
                FROM professionals_specializations 
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(ProfessionalSpecialization::from)
    }

    async fn update(
        &self,
        professional_specialization: &ProfessionalSpecialization,
    ) -> AppResult<()> {
        sqlx::query!(
            "UPDATE professionals_specializations 
                SET s_name = $2
                WHERE id = $1",
            professional_specialization.id,
            professional_specialization.name
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "DELETE FROM professionals_specializations WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }
}
