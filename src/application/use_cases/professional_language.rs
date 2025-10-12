use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{app_error::AppResult, entities::professional_language::ProfessionalLanguage};

#[async_trait]
pub trait ProfessionalLanguagePersistence: Send + Sync {
    async fn create(&self, professional_language: &ProfessionalLanguage) -> AppResult<()>;

    async fn read_all(&self) -> AppResult<Vec<ProfessionalLanguage>>;

    async fn read_single(&self, id: Uuid) -> AppResult<ProfessionalLanguage>;

    async fn update(&self, professional_language: &ProfessionalLanguage) -> AppResult<()>;

    async fn delete(&self, id: Uuid) -> AppResult<()>;
}

#[derive(Clone)]
pub struct ProfessionalLanguageUseCases {
    persistence: Arc<dyn ProfessionalLanguagePersistence>,
}

impl ProfessionalLanguageUseCases {
    pub fn new(persistence: Arc<dyn ProfessionalLanguagePersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn create(&self, professional_language: &ProfessionalLanguage) -> AppResult<()> {
        info!("Attempting create profiessional language...");

        self.persistence.create(professional_language).await?;

        info!("Professional language created.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn read_all(&self) -> AppResult<Vec<ProfessionalLanguage>> {
        self.persistence.read_all().await
    }

    #[instrument(skip(self))]
    pub async fn read_single(&self, id: Uuid) -> AppResult<ProfessionalLanguage> {
        self.persistence.read_single(id).await
    }

    #[instrument(skip(self))]
    pub async fn update(&self, professional_language: &ProfessionalLanguage) -> AppResult<()> {
        info!("Attempting update professional language...");

        self.persistence.update(professional_language).await?;

        info!("Professional language updated.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        info!("Attempting delete professional language...");

        self.persistence.delete(id).await?;

        info!("Professional language deleted.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use crate::app_error::AppError;

    use super::*;

    struct MockProfessionalLanguagePersistence;

    #[async_trait]
    impl ProfessionalLanguagePersistence for MockProfessionalLanguagePersistence {
        async fn create(&self, professional_language: &ProfessionalLanguage) -> AppResult<()> {
            if professional_language.id.is_some() {
                return Err(AppError::Internal(
                    "professional id must be None when creating".into(),
                ));
            }

            Ok(())
        }

        async fn read_all(&self) -> AppResult<Vec<ProfessionalLanguage>> {
            Ok(vec![])
        }

        async fn read_single(&self, _id: Uuid) -> AppResult<ProfessionalLanguage> {
            Ok(ProfessionalLanguage {
                id: Some(Uuid::new_v4()),
                professional_id: Some(Uuid::new_v4()),
                name: String::from("Coco"),
                created_at: None,
            })
        }

        async fn update(&self, professional_language: &ProfessionalLanguage) -> AppResult<()> {
            assert!(professional_language.id.is_some());

            Ok(())
        }

        async fn delete(&self, _id: Uuid) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn create_works() {
        let use_cases =
            ProfessionalLanguageUseCases::new(Arc::new(MockProfessionalLanguagePersistence));

        let result = use_cases
            .create(&ProfessionalLanguage {
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
        let use_cases =
            ProfessionalLanguageUseCases::new(Arc::new(MockProfessionalLanguagePersistence));

        let result = use_cases
            .create(&ProfessionalLanguage {
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
        let use_cases =
            ProfessionalLanguageUseCases::new(Arc::new(MockProfessionalLanguagePersistence));

        let result = use_cases.read_all().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_single_works() {
        let use_cases =
            ProfessionalLanguageUseCases::new(Arc::new(MockProfessionalLanguagePersistence));

        let result = use_cases.read_single(Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_works() {
        let use_cases =
            ProfessionalLanguageUseCases::new(Arc::new(MockProfessionalLanguagePersistence));

        let result = use_cases
            .update(&ProfessionalLanguage {
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
        let use_cases =
            ProfessionalLanguageUseCases::new(Arc::new(MockProfessionalLanguagePersistence));

        let result = use_cases.delete(Uuid::new_v4()).await;

        assert!(result.is_ok());
    }
}
