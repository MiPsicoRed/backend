use std::fmt::Debug;

use uuid::Uuid;

use crate::entities::gender::Gender;

pub struct Professional {
    pub id: Option<Uuid>, // we option this so we can use the same type for update and create but aside that on_create it should never be None
    pub user_id: Option<Uuid>, // we option this so we don't need to pass it for update, as once created we can't modify the user
    pub gender: Gender,
    pub birthdate: Option<chrono::NaiveDate>,
    pub license_number: Option<String>,
    pub bio: Option<String>,
    pub education: Option<String>,
    pub experience_years: Option<i32>,
    pub hourly_rate: Option<f32>,
    pub accepts_insurance: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Debug for Professional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Professional")
            .field("id", &self.id)
            .field("user_id", &self.user_id)
            .field("created_at", &self.created_at)
            .finish()
    }
}
