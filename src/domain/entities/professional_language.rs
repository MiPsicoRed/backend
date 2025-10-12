use uuid::Uuid;

#[derive(Debug)]
pub struct ProfessionalLanguage {
    pub id: Option<Uuid>, // we option this so we can use the same type for update and create but aside that on_create it should never be None
    pub professional_id: Option<Uuid>, // we option this so we don't need to pass it for update, as once created we can't modify the user
    pub name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}
