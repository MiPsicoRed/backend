use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::{email::EmailKind, user_token::UserToken},
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
    /// Adds a user_token to the database and returns the inserted Token
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

    /// Checks if the given user has a token already created, returns the token if exists, returns error if the user does
    /// not exist, returns none if the user does not have a token or it has expired
    async fn check_user_token(&self, user_id: &Uuid) -> AppResult<Option<UserTokenDb>> {
        let now = chrono::Utc::now().naive_utc();

        let token = sqlx::query_as!(
            UserTokenDb,
            r#"
                SELECT ut.id, ut.user_id, ut.token, ut.expires_at, ut.created_at
                FROM user_tokens ut
                INNER JOIN users u ON ut.user_id = u.id
                WHERE ut.user_id = $1 AND ut.expires_at > $2
                ORDER BY ut.created_at DESC
                LIMIT 1
            "#,
            user_id,
            now
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(token)
    }

    /// Fetches the email of a user by id.
    async fn get_user_email(&self, user_id: &Uuid) -> AppResult<String> {
        let email = sqlx::query_scalar!(
            r#"
                SELECT email
                FROM users
                WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(email)
    }

    /// Adds the a verification email to the database with the given params
    async fn add_verification_email(&self, from: &str, to: &str, body: &str) -> AppResult<()> {
        let uuid = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO emails (id, from_mail, to_mail, mail_subject, mail_body, email_kind)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            uuid,
            from,
            to,
            "Please Verify your Account",
            body,
            EmailKind::Verification.to_id()
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }
}
