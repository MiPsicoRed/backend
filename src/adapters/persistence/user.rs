use async_trait::async_trait;
use chrono::NaiveDateTime;
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
    pub email: String,
    pub verified: Option<bool>,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
}

impl From<UserDb> for User {
    fn from(user_db: UserDb) -> Self {
        User {
            id: user_db.id,
            username: user_db.username,
            email: user_db.email,
            verified: user_db.verified,
            password_hash: user_db.password_hash,
            created_at: user_db.created_at,
        }
    }
}

#[async_trait]
impl UserPersistence for PostgresPersistence {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
            uuid,
            username,
            email,
            password_hash
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn get_user_by_username(&self, username: &str) -> AppResult<User> {
        sqlx::query_as!(
            UserDb,
            "SELECT id, username, email, verified, password_hash, created_at 
            FROM users 
            WHERE username = $1",
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(User::from)
    }

    async fn get_all_users(&self) -> AppResult<Vec<User>> {
        sqlx::query_as!(
            UserDb,
            r#"SELECT id, username, email, verified, ''::text as "password_hash!", created_at
                FROM users"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|users| users.into_iter().map(User::from).collect())
    }
}
