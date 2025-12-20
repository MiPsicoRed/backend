use chrono::NaiveDate;
use uuid::Uuid;

use crate::entities::{gender::Gender, sexual_orientation::SexualOrientation, user::User};

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

impl From<&User> for Patient {
    fn from(value: &User) -> Self {
        Self {
            id: None,
            user_id: Some(value.id),
            gender: Gender::default(),
            sexual_orientation: SexualOrientation::default(),
            birthdate: Some(
                NaiveDate::from_ymd_opt(1990, 1, 1).expect("Could not parse default Patient Date"),
            ),
            phone: String::from(""),
            emergency_contact_name: None,
            emergency_contact_phone: None,
            insurance_policy_number: None,
            medical_history: None,
            current_medications: None,
            allergies: None,
            created_at: None,
        }
    }
}
