use uuid::Uuid;

use crate::entities::{gender::Gender, sexual_orientation::SexualOrientation};

#[derive(Debug)]
pub struct Patient {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub gender: Gender,
    pub sexual_orientation: SexualOrientation,
    pub birthdate: Option<chrono::NaiveDate>,
    pub phone: String,
    pub emergency_contact_name: String,
    pub emergency_contact_phone: String,
    pub insurance_policy_number: String,
    pub medical_history: String,
    pub current_medications: String,
    pub allergies: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}
