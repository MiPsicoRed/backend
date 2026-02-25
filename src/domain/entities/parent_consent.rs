use uuid::Uuid;

#[derive(Debug)]
pub struct ParentConsent {
    pub id: Option<Uuid>,
    pub patient_id: Uuid,
    pub guardian_name: String,
    pub guardian_id_document: String,
    pub signature_data: String,
    pub consent_certificate_id: String,
    pub signed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
