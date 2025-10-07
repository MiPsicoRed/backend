use async_trait::async_trait;
use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::user::User,
    use_cases::user::UserPersistence,
};

// User struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserDb {
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

impl From<UserDb> for User {
    fn from(user_db: UserDb) -> Self {
        User {
            id: user_db.id,
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
    ) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO users (id, username, usersurname, email, phone, birthdate, password_hash) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            uuid,
            username,
            usersurname,
            email,
            phone,
            birthdate,
            password_hash
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn get_user_by_email(&self, email: &str) -> AppResult<User> {
        sqlx::query_as!(
            UserDb,
            "SELECT id, username, usersurname, email, phone, birthdate, verified, password_hash, created_at 
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
            r#"SELECT id, username, usersurname, email, phone, birthdate, verified, ''::text as "password_hash!", created_at
                FROM users"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|users| users.into_iter().map(User::from).collect())
    }
}
