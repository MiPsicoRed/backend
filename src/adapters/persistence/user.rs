use std::fmt::Display;

use async_trait::async_trait;
use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::user::{Role, User},
    use_cases::user::UserPersistence,
};

// User struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserDb {
    pub id: Uuid,
    pub role: RoleDb,
    pub username: String,
    pub usersurname: String,
    pub email: String,
    pub phone: String,
    pub birthdate: Option<chrono::NaiveDate>,
    pub verified: Option<bool>,
    pub password_hash: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<UserDb> for User {
    fn from(user_db: UserDb) -> Self {
        User {
            id: user_db.id,
            role: user_db.role.into(),
            username: user_db.username,
            usersurname: user_db.usersurname,
            email: user_db.email,
            phone: user_db.phone,
            birthdate: user_db.birthdate,
            verified: user_db.verified,
            password_hash: user_db.password_hash,
            created_at: user_db.created_at,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum RoleDb {
    #[default]
    Patient,
    Professional,
    Admin,
}

impl From<RoleDb> for Role {
    fn from(value: RoleDb) -> Self {
        match value {
            RoleDb::Patient => Role::Patient,
            RoleDb::Professional => Role::Professional,
            RoleDb::Admin => Role::Admin,
        }
    }
}

impl Display for RoleDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Role::from(*self).fmt(f)
    }
}

impl RoleDb {
    pub fn to_id(self) -> i32 {
        Role::from(self).to_id()
    }

    pub fn from_id(id: i32) -> Option<Self> {
        Role::from_id(id).map(|kind| match kind {
            Role::Patient => RoleDb::Patient,
            Role::Professional => RoleDb::Professional,
            Role::Admin => RoleDb::Admin,
        })
    }
}

impl From<i32> for RoleDb {
    fn from(id: i32) -> Self {
        RoleDb::from_id(id).unwrap_or_default()
    }
}

#[async_trait]
impl UserPersistence for PostgresPersistence {
    async fn create_user(
        &self,
        username: &str,
        usersurname: &str,
        email: &str,
        phone: &str,
        birthdate: Option<NaiveDate>,
        password_hash: &str,
    ) -> AppResult<User> {
        let uuid = Uuid::new_v4();

        sqlx::query_as!(
        UserDb,
            "INSERT INTO users (id, role_id, username, usersurname, email, phone, birthdate, password_hash) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, role_id as role, username, usersurname, email, phone, birthdate, verified, password_hash, created_at",
            uuid,
            RoleDb::default().to_id(),
            username,
            usersurname,
            email,
            phone,
            birthdate,
            password_hash
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(User::from)
    }

    async fn get_user_by_email(&self, email: &str) -> AppResult<User> {
        sqlx::query_as!(
            UserDb,
            "SELECT id, role_id as role, username, usersurname, email, phone, birthdate, verified, password_hash, created_at 
            FROM users 
            WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(User::from)
    }

    async fn get_all_users(&self) -> AppResult<Vec<User>> {
        sqlx::query_as!(
            UserDb,
            r#"SELECT id, role_id as role, username, usersurname, email, phone, birthdate, verified, ''::text as "password_hash!", created_at
                FROM users"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|users| users.into_iter().map(User::from).collect())
    }
}
