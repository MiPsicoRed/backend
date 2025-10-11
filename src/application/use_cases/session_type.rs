use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{app_error::AppResult, entities::session_type::SessionType};

#[async_trait]
pub trait SessionTypePersistence: Send + Sync {
    async fn create(&self, name: &str) -> AppResult<()>;

    async fn read_all(&self) -> AppResult<Vec<SessionType>>;

    async fn read_single(&self, id: Uuid) -> AppResult<SessionType>;

    async fn update(&self, id: Uuid, name: &str) -> AppResult<()>;

    async fn delete(&self, id: Uuid) -> AppResult<()>;
}

#[derive(Clone)]
pub struct SessionTypeUseCases {
    persistence: Arc<dyn SessionTypePersistence>,
}

impl SessionTypeUseCases {
    pub fn new(persistence: Arc<dyn SessionTypePersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn create(&self, name: &str) -> AppResult<()> {
        info!("Attempting create session type...");

        self.persistence.create(name).await?;

        info!("Sesssion type created.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn read_all(&self) -> AppResult<Vec<SessionType>> {
        self.persistence.read_all().await
    }

    #[instrument(skip(self))]
    pub async fn read_single(&self, id: Uuid) -> AppResult<SessionType> {
        self.persistence.read_single(id).await
    }

    #[instrument(skip(self))]
    pub async fn update(&self, id: Uuid, name: &str) -> AppResult<()> {
        info!("Attempting update session type...");

        self.persistence.update(id, name).await?;

        info!("Session type updated.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        info!("Attempting delete session type...");

        self.persistence.delete(id).await?;

        info!("Session type deleted.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    #[allow(dead_code)]
    struct MockSessionTypePersistence;

    #[async_trait]
    impl SessionTypePersistence for MockSessionTypePersistence {
        async fn create(&self, name: &str) -> AppResult<()> {
            assert!(!name.is_empty());

            Ok(())
        }

        async fn read_all(&self) -> AppResult<Vec<SessionType>> {
            Ok(vec![])
        }

        async fn read_single(&self, _id: Uuid) -> AppResult<SessionType> {
            Ok(SessionType {
                id: Uuid::new_v4(),
                name: String::from("Session Type Name"),
                created_at: Some(chrono::Utc::now().naive_utc()),
            })
        }

        async fn update(&self, _id: Uuid, name: &str) -> AppResult<()> {
            assert!(!name.is_empty());

            Ok(())
        }

        async fn delete(&self, _id: Uuid) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn create_works() {
        let use_cases = SessionTypeUseCases::new(Arc::new(MockSessionTypePersistence));

        let result = use_cases.create("Coco").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_all_works() {
        let use_cases = SessionTypeUseCases::new(Arc::new(MockSessionTypePersistence));

        let result = use_cases.read_all().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_single_works() {
        let use_cases = SessionTypeUseCases::new(Arc::new(MockSessionTypePersistence));

        let result = use_cases.read_single(Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_works() {
        let use_cases = SessionTypeUseCases::new(Arc::new(MockSessionTypePersistence));

        let result = use_cases.update(Uuid::new_v4(), "Coco").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_works() {
        let use_cases = SessionTypeUseCases::new(Arc::new(MockSessionTypePersistence));

        let result = use_cases.delete(Uuid::new_v4()).await;

        assert!(result.is_ok());
    }
}
