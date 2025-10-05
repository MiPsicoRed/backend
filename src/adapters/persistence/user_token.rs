use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::user_token::UserToken,
    use_cases::user_token::UserTokenPersistence,
};

// User struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserTokenDb {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

impl From<UserTokenDb> for UserToken {
    fn from(user_token_db: UserTokenDb) -> Self {
        UserToken {
            id: user_token_db.id,
            user_id: user_token_db.user_id,
            token: user_token_db.token,
            expires_at: user_token_db.expires_at,
            created_at: user_token_db.created_at,
        }
    }
}

#[async_trait]
impl UserTokenPersistence for PostgresPersistence {
    async fn add_user_token(
        &self,
        user_id: Uuid,
        token: String,
        expires_at: NaiveDateTime,
    ) -> AppResult<UserTokenDb> {
        let uuid = Uuid::new_v4();

        let token = sqlx::query_as!(
            UserTokenDb,
            r#"
            INSERT INTO user_tokens (id, user_id, token, expires_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, token, expires_at, created_at
            "#,
            uuid,
            user_id,
            token,
            expires_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(token)
    }
}
