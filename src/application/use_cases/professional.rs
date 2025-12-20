use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    app_error::AppResult, dtos::professional::selector::ProfessionalSelectorDTO,
    entities::professional::Professional,
};

#[async_trait]
pub trait ProfessionalPersistence: Send + Sync {
    async fn create(&self, professional: &Professional) -> AppResult<()>;

    async fn read_all(&self) -> AppResult<Vec<Professional>>;

    async fn read_single(&self, id: &Uuid) -> AppResult<Professional>;

    async fn read_by_user(&self, id: &Uuid) -> AppResult<Professional>;

    async fn update(&self, professional: &Professional) -> AppResult<()>;

    async fn delete(&self, id: &Uuid) -> AppResult<()>;

    async fn selector(&self) -> AppResult<Vec<ProfessionalSelectorDTO>>;
}

#[derive(Clone)]
pub struct ProfessionalUseCases {
    persistence: Arc<dyn ProfessionalPersistence>,
}

impl ProfessionalUseCases {
    pub fn new(persistence: Arc<dyn ProfessionalPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn create(&self, professional: &Professional) -> AppResult<()> {
        info!("Attempting create profiessional...");

        self.persistence.create(professional).await?;

        info!("Professional created.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn read_all(&self) -> AppResult<Vec<Professional>> {
        self.persistence.read_all().await
    }

    #[instrument(skip(self))]
    pub async fn read_single(&self, id: &Uuid) -> AppResult<Professional> {
        self.persistence.read_single(id).await
    }

    #[instrument(skip(self))]
    pub async fn read_by_user(&self, user_id: &Uuid) -> AppResult<Professional> {
        self.persistence.read_by_user(user_id).await
    }

    #[instrument(skip(self))]
    pub async fn update(&self, professional: &Professional) -> AppResult<()> {
        info!("Attempting update professional...");

        self.persistence.update(professional).await?;

        info!("Professional updated.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: &Uuid) -> AppResult<()> {
        info!("Attempting delete professional...");

        self.persistence.delete(id).await?;

        info!("Professional deleted.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn selector(&self) -> AppResult<Vec<ProfessionalSelectorDTO>> {
        self.persistence.selector().await
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use crate::{app_error::AppError, entities::gender::Gender};

    use super::*;

    struct MockProfessionalPersistence;

    #[async_trait]
    impl ProfessionalPersistence for MockProfessionalPersistence {
        async fn create(&self, professional: &Professional) -> AppResult<()> {
            if professional.id.is_some() {
                return Err(AppError::Internal(
                    "professional id must be None when creating".into(),
                ));
            }

            Ok(())
        }

        async fn read_all(&self) -> AppResult<Vec<Professional>> {
            Ok(vec![])
        }

        async fn read_single(&self, _id: &Uuid) -> AppResult<Professional> {
            Ok(Professional {
                id: Some(Uuid::new_v4()),
                user_id: Some(Uuid::new_v4()),
                gender: Gender::Male,
                birthdate: Some(chrono::NaiveDate::from_ymd_opt(2000, 11, 9).unwrap()),
                license_number: None,
                bio: None,
                education: None,
                experience_years: None,
                hourly_rate: None,
                accepts_insurance: false,
                created_at: None,
            })
        }

        async fn read_by_user(&self, _id: &Uuid) -> AppResult<Professional> {
            Ok(Professional {
                id: Some(Uuid::new_v4()),
                user_id: Some(Uuid::new_v4()),
                gender: Gender::Male,
                birthdate: Some(chrono::NaiveDate::from_ymd_opt(2000, 11, 9).unwrap()),
                license_number: None,
                bio: None,
                education: None,
                experience_years: None,
                hourly_rate: None,
                accepts_insurance: false,
                created_at: None,
            })
        }

        async fn update(&self, professional: &Professional) -> AppResult<()> {
            assert!(professional.id.is_some());

            Ok(())
        }

        async fn delete(&self, _id: &Uuid) -> AppResult<()> {
            Ok(())
        }

        async fn selector(&self) -> AppResult<Vec<ProfessionalSelectorDTO>> {
            Ok(vec![])
        }
    }

    #[tokio::test]
    async fn create_works() {
        let use_cases = ProfessionalUseCases::new(Arc::new(MockProfessionalPersistence));

        let result = use_cases
            .create(&Professional {
                id: None,
                user_id: Some(Uuid::new_v4()),
                gender: Gender::Male,
                birthdate: Some(chrono::NaiveDate::from_ymd_opt(2000, 11, 9).unwrap()),
                license_number: None,
                bio: None,
                education: None,
                experience_years: None,
                hourly_rate: None,
                accepts_insurance: false,
                created_at: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn create_with_id_fails() {
        let use_cases = ProfessionalUseCases::new(Arc::new(MockProfessionalPersistence));

        let result = use_cases
            .create(&Professional {
                id: Some(Uuid::new_v4()),
                user_id: Some(Uuid::new_v4()),
                gender: Gender::Male,
                birthdate: Some(chrono::NaiveDate::from_ymd_opt(2000, 11, 9).unwrap()),
                license_number: None,
                bio: None,
                education: None,
                experience_years: None,
                hourly_rate: None,
                accepts_insurance: false,
                created_at: None,
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn read_all_works() {
        let use_cases = ProfessionalUseCases::new(Arc::new(MockProfessionalPersistence));

        let result = use_cases.read_all().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_single_works() {
        let use_cases = ProfessionalUseCases::new(Arc::new(MockProfessionalPersistence));

        let result = use_cases.read_single(&Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_by_user_works() {
        let use_cases = ProfessionalUseCases::new(Arc::new(MockProfessionalPersistence));

        let result = use_cases.read_by_user(&Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_works() {
        let use_cases = ProfessionalUseCases::new(Arc::new(MockProfessionalPersistence));

        let result = use_cases
            .update(&Professional {
                id: Some(Uuid::new_v4()),
                user_id: None,
                gender: Gender::Male,
                birthdate: Some(chrono::NaiveDate::from_ymd_opt(2000, 11, 9).unwrap()),
                license_number: None,
                bio: None,
                education: None,
                experience_years: None,
                hourly_rate: None,
                accepts_insurance: false,
                created_at: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_works() {
        let use_cases = ProfessionalUseCases::new(Arc::new(MockProfessionalPersistence));

        let result = use_cases.delete(&Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn selector_works() {
        let use_cases = ProfessionalUseCases::new(Arc::new(MockProfessionalPersistence));

        let result = use_cases.selector().await;

        assert!(result.is_ok());
    }
}
