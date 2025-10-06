use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    adapters::{
        persistence::user_token::UserTokenDb,
        utils::verification_token::generate_verification_token,
    },
    app_error::{AppError, AppResult},
};

#[async_trait]
pub trait UserTokenPersistence: Send + Sync {
    async fn add_user_token(
        &self,
        user_id: Uuid,
        token: String,
        expires_at: NaiveDateTime,
    ) -> AppResult<UserTokenDb>;
}

#[derive(Clone)]
pub struct UserTokenUseCases {
    email_service: Arc<dyn UserTokenEmailService>,
    persistence: Arc<dyn UserTokenPersistence>,
}

#[async_trait]
pub trait UserTokenEmailService: Send + Sync {
    async fn send_email(
        &self,
        from: &str,
        to: &[String],
        subject: &str,
        email_html: &str,
    ) -> AppResult<()>;
}

impl UserTokenUseCases {
    pub fn new(
        email_service: Arc<dyn UserTokenEmailService>,
        persistence: Arc<dyn UserTokenPersistence>,
    ) -> Self {
        Self {
            email_service,
            persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn generate_token(&self, user_id: &str) -> AppResult<UserTokenDb> {
        info!("Attempting to generate token...");

        let user_uuid = Uuid::parse_str(user_id)
            .map_err(|_| AppError::Internal("Invalid UUID string for given user_id:".into()))?;

        let token = generate_verification_token();

        // expiry date = 5 days from now
        let token_expiry_date = (chrono::Utc::now() + chrono::Duration::days(5)).naive_utc();

        let token = self
            .persistence
            .add_user_token(user_uuid, token, token_expiry_date)
            .await?;

        info!("User token generated.");

        Ok(token)
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use uuid::Uuid;

    use super::*;

    struct MockUserTokenPersistence;

    #[async_trait]
    impl UserTokenPersistence for MockUserTokenPersistence {
        async fn add_user_token(
            &self,
            user_id: Uuid,
            token: String,
            expires_at: NaiveDateTime,
        ) -> AppResult<UserTokenDb> {
            Ok(UserTokenDb {
                id: Uuid::new_v4(),
                user_id,
                token,
                expires_at: Some(expires_at),
                created_at: None,
            })
        }
    }

    struct MockUserTokenEmailService;

    #[async_trait]
    impl UserTokenEmailService for MockUserTokenEmailService {
        async fn send_email(
            &self,
            _from: &str,
            _to: &[String],
            _subject: &str,
            _email_html: &str,
        ) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn generate_token_works() {
        let user_token_use_cases = UserTokenUseCases::new(
            Arc::new(MockUserTokenEmailService),
            Arc::new(MockUserTokenPersistence),
        );

        let result = user_token_use_cases
            .generate_token("24d7fa6e-4c52-40ff-ad25-5271e8c48345") // this does not mean the user is in the db, this is just a valid uuid
            .await;

        assert!(result.is_ok());
    }
}
