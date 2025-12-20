use uuid::Uuid;

#[derive(Debug)]
pub struct ProfessionalSelectorDTO {
    pub professional_id: Uuid,
    pub name: String,
}
