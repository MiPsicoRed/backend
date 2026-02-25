use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{app_error::AppResult, domain::entities::parent_consent::ParentConsent};

#[async_trait]
pub trait ParentConsentPersistence: Send + Sync {
    async fn create(&self, consent: &ParentConsent) -> AppResult<()>;
    async fn read_by_patient(&self, patient_id: &Uuid) -> AppResult<Option<ParentConsent>>;
}

#[derive(Clone)]
pub struct ParentConsentUseCases {
    persistence: Arc<dyn ParentConsentPersistence>,
}

impl ParentConsentUseCases {
    pub fn new(persistence: Arc<dyn ParentConsentPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn create(&self, consent: &ParentConsent) -> AppResult<()> {
        info!("Creating parent consent...");
        self.persistence.create(consent).await?;
        info!("Parent consent created.");
        Ok(())
    }
}
