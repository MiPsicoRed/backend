use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::{app_error::AppResult, entities::mood_log::MoodLog};

#[async_trait]
pub trait MoodLogPersistence: Send + Sync {
    async fn create(&self, mood_log: &MoodLog) -> AppResult<()>;
    async fn read_by_patient(&self, patient_id: &Uuid) -> AppResult<Vec<MoodLog>>;
    async fn read_shared_by_patient(&self, patient_id: &Uuid) -> AppResult<Vec<MoodLog>>;
    async fn delete(&self, id: &Uuid) -> AppResult<()>;
}

#[derive(Clone)]
pub struct MoodLogUseCases {
    persistence: Arc<dyn MoodLogPersistence>,
}

impl MoodLogUseCases {
    pub fn new(persistence: Arc<dyn MoodLogPersistence>) -> Self {
        Self { persistence }
    }

    pub async fn create(&self, mood_log: &MoodLog) -> AppResult<()> {
        self.persistence.create(mood_log).await
    }

    pub async fn read_by_patient(&self, patient_id: &Uuid) -> AppResult<Vec<MoodLog>> {
        self.persistence.read_by_patient(patient_id).await
    }

    pub async fn read_shared_by_patient(&self, patient_id: &Uuid) -> AppResult<Vec<MoodLog>> {
        self.persistence.read_shared_by_patient(patient_id).await
    }

    pub async fn delete(&self, id: &Uuid) -> AppResult<()> {
        self.persistence.delete(id).await
    }
}
