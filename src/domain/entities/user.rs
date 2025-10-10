use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub role: Role,
    pub username: String,
    pub usersurname: String,
    pub email: String,
    pub verified: Option<bool>,
    pub password_hash: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Role {
    #[default]
    Patient,
    Professional,
    Admin,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Role::Patient => write!(f, "Patient"),
            Role::Professional => write!(f, "Professional"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}

impl Role {
    pub const ALL: &'static [Self] = &[Self::Patient, Self::Professional, Self::Admin];

    pub fn to_id(&self) -> i32 {
        match self {
            Role::Patient => 1,
            Role::Professional => 2,
            Role::Admin => 3,
        }
    }

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(Role::Patient),
            2 => Some(Role::Professional),
            3 => Some(Role::Admin),
            _ => None,
        }
    }
}
