use uuid::Uuid;

use crate::entities::{gender::Gender, sexual_orientation::SexualOrientation};

#[derive(Debug)]
pub struct Patient {
    pub id: Option<Uuid>, // we option this so we can use the same type for update and create but aside that on_create it should never be None
    pub user_id: Option<Uuid>,
    pub gender: Gender,
    pub sexual_orientation: SexualOrientation,
    pub birthdate: Option<chrono::NaiveDate>,
    pub phone: String,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub insurance_policy_number: Option<String>,
    pub medical_history: Option<String>,
    pub current_medications: Option<String>,
    pub allergies: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
