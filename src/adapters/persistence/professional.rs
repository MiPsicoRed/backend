use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::{gender::Gender, professional::Professional},
    use_cases::professional::ProfessionalPersistence,
};

// Professional struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ProfessionalDb {
    pub id: Uuid,
    pub user_id: Uuid,
    pub gender_id: i32,
    pub birthdate: chrono::NaiveDate,
    pub license_number: Option<String>,
    pub bio: Option<String>,
    pub education: Option<String>,
    pub experience_years: Option<i32>,
    pub hourly_rate: Option<f32>,
    pub accepts_insurance: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<ProfessionalDb> for Professional {
    fn from(professional_db: ProfessionalDb) -> Self {
        Professional {
            id: Some(professional_db.id),
            user_id: Some(professional_db.user_id),
            gender: Gender::from_id(professional_db.gender_id).unwrap_or_default(),
            birthdate: Some(professional_db.birthdate),
            license_number: professional_db.license_number,
            bio: professional_db.bio,
            education: professional_db.education,
            experience_years: professional_db.experience_years,
            hourly_rate: professional_db.hourly_rate,
            accepts_insurance: professional_db.accepts_insurance,
            created_at: professional_db.created_at,
        }
    }
}

#[async_trait]
impl ProfessionalPersistence for PostgresPersistence {
    async fn create(&self, professional: &Professional) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        // this is not needed since in professionals user_id is a proper foreign constraint, not as in patients
        // if let Some(uid) = professional.user_id {
        //     let exists =
        //         sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)", uid)
        //             .fetch_one(&self.pool)
        //             .await
        //             .map_err(AppError::Database)?
        //             .unwrap_or(false);

        //     if !exists {
        //         return Err(AppError::Internal(String::from("User does not exist")));
        //     }
        // }

        sqlx::query!(
                "INSERT INTO professionals (id, user_id, gender_id, birthdate, license_number, bio, education, experience_years, hourly_rate, accepts_insurance) 
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                uuid,
                professional.user_id,
                professional.gender.to_id(),
                professional.birthdate,
                professional.license_number,
                professional.bio,
                professional.education,
                professional.experience_years,
                professional.hourly_rate,
                professional.accepts_insurance
            )
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }

    async fn read_all(&self) -> AppResult<Vec<Professional>> {
        sqlx::query_as!(
            ProfessionalDb,
            r#"
                SELECT id, user_id, gender_id, birthdate, license_number, bio, education, experience_years, hourly_rate, accepts_insurance, created_at
                FROM professionals
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|professionals| professionals.into_iter().map(Professional::from).collect())
    }

    async fn read_single(&self, id: Uuid) -> AppResult<Professional> {
        sqlx::query_as!(
            ProfessionalDb,
            r#"
                SELECT id, user_id, gender_id, birthdate, license_number, bio, education, experience_years, hourly_rate, accepts_insurance, created_at
                FROM professionals 
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(Professional::from)
    }

    async fn update(&self, professional: &Professional) -> AppResult<()> {
        sqlx::query!(
            "UPDATE professionals 
                SET gender_id = $2, birthdate = $3, license_number = $4, bio = $5, education = $6, experience_years = $7, hourly_rate = $8, accepts_insurance = $9
                WHERE id = $1",
            professional.id,
            professional.gender.to_id(),
            professional.birthdate,
            professional.license_number,
            professional.bio,
            professional.education,
            professional.experience_years,
            professional.hourly_rate,
            professional.accepts_insurance
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM professionals WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }
}
