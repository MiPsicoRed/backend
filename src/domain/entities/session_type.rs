use uuid::Uuid;

#[derive(Debug)]
pub struct SessionType {
    pub id: Uuid,
    pub name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}
