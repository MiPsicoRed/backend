use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub usersurname: String,
    pub email: String,
    pub phone: String,
    pub birthdate: Option<chrono::NaiveDate>,
    pub verified: Option<bool>,
    pub password_hash: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}
