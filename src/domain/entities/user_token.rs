use uuid::Uuid;

#[derive(Debug)]
pub struct UserToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
