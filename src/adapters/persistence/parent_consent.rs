use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    domain::entities::parent_consent::ParentConsent,
    application::use_cases::parent_consent::ParentConsentPersistence,
};

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ParentConsentDb {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub guardian_name: String,
    pub guardian_id_document: String,
    pub signature_data: String,
    pub consent_certificate_id: String,
    pub signed_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<ParentConsentDb> for ParentConsent {
    fn from(db: ParentConsentDb) -> Self {
        Self {
            id: Some(db.id),
            patient_id: db.patient_id,
            guardian_name: db.guardian_name,
            guardian_id_document: db.guardian_id_document,
            signature_data: db.signature_data,
            consent_certificate_id: db.consent_certificate_id,
            signed_at: db.signed_at,
            created_at: db.created_at,
        }
    }
}

#[async_trait]
impl ParentConsentPersistence for PostgresPersistence {
    async fn create(&self, consent: &ParentConsent) -> AppResult<()> {
        let id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO parent_consents (id, patient_id, guardian_name, guardian_id_document, signature_data, consent_certificate_id) 
            VALUES ($1, $2, $3, $4, $5, $6)",
            id,
            consent.patient_id,
            consent.guardian_name,
            consent.guardian_id_document,
            consent.signature_data,
            consent.consent_certificate_id
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn read_by_patient(&self, patient_id: &Uuid) -> AppResult<Option<ParentConsent>> {
        let result = sqlx::query_as!(
            ParentConsentDb,
            "SELECT * FROM parent_consents WHERE patient_id = $1",
            patient_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(result.map(ParentConsent::from))
    }
}
