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
    persistence: Arc<dyn UserTokenPersistence>,
}

impl UserTokenUseCases {
    pub fn new(persistence: Arc<dyn UserTokenPersistence>) -> Self {
        Self { persistence }
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

    #[tokio::test]
    async fn generate_token_works() {
        let user_token_use_cases = UserTokenUseCases::new(Arc::new(MockUserTokenPersistence));

        let result = user_token_use_cases
            .generate_token("24d7fa6e-4c52-40ff-ad25-5271e8c48345") // this does not mean the user is in the db, this is just a valid uuid
            .await;

        assert!(result.is_ok());
    }
}
