use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct UserOnboarding {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_type: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub birthdate: Option<chrono::NaiveDate>,
    pub reason: Option<String>,
    pub experience: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserConsent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub is_monoparental: bool,
    pub guardian_name: Option<String>,
    pub guardian_id_document: Option<String>,
    pub signature: Option<String>,
    pub guardian2_name: Option<String>,
    pub guardian2_id_document: Option<String>,
    pub signature2: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}
