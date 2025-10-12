use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    app_error::AppResult, entities::professional_specialization::ProfessionalSpecialization,
};

#[async_trait]
pub trait ProfessionalSpecializationPersistence: Send + Sync {
    async fn create(
        &self,
        professional_specialization: &ProfessionalSpecialization,
    ) -> AppResult<()>;

    async fn read_all(&self) -> AppResult<Vec<ProfessionalSpecialization>>;

    async fn read_single(&self, id: Uuid) -> AppResult<ProfessionalSpecialization>;

    async fn update(
        &self,
        professional_specialization: &ProfessionalSpecialization,
    ) -> AppResult<()>;

    async fn delete(&self, id: Uuid) -> AppResult<()>;
}

#[derive(Clone)]
pub struct ProfessionalSpecializationUseCases {
    persistence: Arc<dyn ProfessionalSpecializationPersistence>,
}

impl ProfessionalSpecializationUseCases {
    pub fn new(persistence: Arc<dyn ProfessionalSpecializationPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn create(
        &self,
        professional_specialization: &ProfessionalSpecialization,
    ) -> AppResult<()> {
        info!("Attempting create profiessional language...");

        self.persistence.create(professional_specialization).await?;

        info!("professional specialization created.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn read_all(&self) -> AppResult<Vec<ProfessionalSpecialization>> {
        self.persistence.read_all().await
    }

    #[instrument(skip(self))]
    pub async fn read_single(&self, id: Uuid) -> AppResult<ProfessionalSpecialization> {
        self.persistence.read_single(id).await
    }

    #[instrument(skip(self))]
    pub async fn update(
        &self,
        professional_specialization: &ProfessionalSpecialization,
    ) -> AppResult<()> {
        info!("Attempting update professional specialization...");

        self.persistence.update(professional_specialization).await?;

        info!("professional specialization updated.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        info!("Attempting delete professional specialization...");

        self.persistence.delete(id).await?;

        info!("professional specialization deleted.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use crate::app_error::AppError;

    use super::*;

    struct MockProfessionalSpecializationPersistence;

    #[async_trait]
    impl ProfessionalSpecializationPersistence for MockProfessionalSpecializationPersistence {
        async fn create(
            &self,
            professional_specialization: &ProfessionalSpecialization,
        ) -> AppResult<()> {
            if professional_specialization.id.is_some() {
                return Err(AppError::Internal(
                    "professional id must be None when creating".into(),
                ));
            }

            Ok(())
        }

        async fn read_all(&self) -> AppResult<Vec<ProfessionalSpecialization>> {
            Ok(vec![])
        }

        async fn read_single(&self, _id: Uuid) -> AppResult<ProfessionalSpecialization> {
            Ok(ProfessionalSpecialization {
                id: Some(Uuid::new_v4()),
                professional_id: Some(Uuid::new_v4()),
                name: String::from("Coco"),
                created_at: None,
            })
        }

        async fn update(
            &self,
            professional_specialization: &ProfessionalSpecialization,
        ) -> AppResult<()> {
            assert!(professional_specialization.id.is_some());

            Ok(())
        }

        async fn delete(&self, _id: Uuid) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn create_works() {
        let use_cases = ProfessionalSpecializationUseCases::new(Arc::new(
            MockProfessionalSpecializationPersistence,
        ));

        let result = use_cases
            .create(&ProfessionalSpecialization {
                id: None,
                professional_id: Some(Uuid::new_v4()),
                name: String::from("Coco"),
                created_at: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn create_with_id_fails() {
        let use_cases = ProfessionalSpecializationUseCases::new(Arc::new(
            MockProfessionalSpecializationPersistence,
        ));

        let result = use_cases
            .create(&ProfessionalSpecialization {
                id: Some(Uuid::new_v4()),
                professional_id: Some(Uuid::new_v4()),
                name: String::from("Coco"),
                created_at: None,
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn read_all_works() {
        let use_cases = ProfessionalSpecializationUseCases::new(Arc::new(
            MockProfessionalSpecializationPersistence,
        ));

        let result = use_cases.read_all().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_single_works() {
        let use_cases = ProfessionalSpecializationUseCases::new(Arc::new(
            MockProfessionalSpecializationPersistence,
        ));

        let result = use_cases.read_single(Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_works() {
        let use_cases = ProfessionalSpecializationUseCases::new(Arc::new(
            MockProfessionalSpecializationPersistence,
        ));

        let result = use_cases
            .update(&ProfessionalSpecialization {
                id: Some(Uuid::new_v4()),
                professional_id: Some(Uuid::new_v4()),
                name: String::from("Coco"),
                created_at: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_works() {
        let use_cases = ProfessionalSpecializationUseCases::new(Arc::new(
            MockProfessionalSpecializationPersistence,
        ));

        let result = use_cases.delete(Uuid::new_v4()).await;

        assert!(result.is_ok());
    }
}
