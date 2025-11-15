use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    adapters::utils::verification_token::generate_verification_token,
    app_error::{AppError, AppResult},
    entities::user_token::UserToken,
};

#[async_trait]
pub trait UserTokenPersistence: Send + Sync {
    async fn add_user_token(
        &self,
        user_id: Uuid,
        token: String,
        expires_at: NaiveDateTime,
    ) -> AppResult<UserToken>;

    async fn check_user_token(&self, user_id: &Uuid) -> AppResult<Option<UserToken>>;

    async fn check_validation_status(&self, user_id: &Uuid) -> AppResult<bool>;

    async fn get_user_email(&self, user_id: &Uuid) -> AppResult<String>;

    async fn add_verification_email(&self, from: &str, to: &str, body: &str) -> AppResult<()>;

    async fn verify_user_token(&self, token: &str) -> AppResult<()>;
}

#[async_trait]
pub trait UserTokenEmailService: Send + Sync {
    /// Returns the 'from' email and the email body
    async fn send_verification_email(
        &self,
        to: &[String],
        token: &str,
    ) -> AppResult<(String, String)>;
}

pub trait UserTokenJwtService: Send + Sync {
    fn validate_token(&self, token: &str) -> AppResult<()>;
}

#[derive(Clone)]
pub struct UserTokenUseCases {
    jwt_service: Arc<dyn UserTokenJwtService>,
    email_service: Arc<dyn UserTokenEmailService>,
    persistence: Arc<dyn UserTokenPersistence>,
}

impl UserTokenUseCases {
    pub fn new(
        jwt_service: Arc<dyn UserTokenJwtService>,
        email_service: Arc<dyn UserTokenEmailService>,
        persistence: Arc<dyn UserTokenPersistence>,
    ) -> Self {
        Self {
            jwt_service,
            email_service,
            persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn generate_token_and_send_mail(&self, user_id: &str) -> AppResult<()> {
        // Flow of this should be:
        // 0 - Check if the user is already validated
        // 1 - Check if there is a non expired token already created for this user
        // 2 - If there is a token go to number 4
        // 3 - Generate a token
        // 4 - Attempt to send verifiaction email
        // 5 - If email is sent correctly save email in the database

        let user_uuid = Uuid::parse_str(user_id)
            .map_err(|_| AppError::Internal("Invalid UUID string for given user_id:".into()))?;

        info!("Checking if user is already verified...");

        let is_already_valid = self.persistence.check_validation_status(&user_uuid).await?;
        if is_already_valid {
            return Err(AppError::Internal(String::from("User is already valid")));
        }

        info!("Checking if user already has a token...");

        let token = if let Some(token) = self.persistence.check_user_token(&user_uuid).await? {
            info!("User already has a token...");
            token
        } else {
            info!("Attempting to generate token...");
            let token = generate_verification_token();

            // expiry date = 5 days from now
            let token_expiry_date = (chrono::Utc::now() + chrono::Duration::days(5)).naive_utc();

            let token = self
                .persistence
                .add_user_token(user_uuid, token, token_expiry_date)
                .await?;

            info!("User token generated.");

            token
        };

        info!("Getting user email");
        let user_email = self.persistence.get_user_email(&user_uuid).await?;
        info!("User email retrieved");

        info!("Sending verification email");
        let email_res = self
            .email_service
            .send_verification_email(std::slice::from_ref(&user_email), &token.token)
            .await?;
        info!("Sent verification email");

        info!("Saving verification email on the database");
        self.persistence
            .add_verification_email(&email_res.0, &user_email, &email_res.1)
            .await?;
        info!("Saved verification email on the database");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn verify_token(&self, token: &str) -> AppResult<()> {
        info!("Attempting to verify token...");

        self.persistence.verify_user_token(token).await?;

        info!("Verified user token");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn validate_token(&self, token: &str) -> AppResult<()> {
        info!("Attempting to validate token...");

        self.jwt_service.validate_token(token)?;

        info!("Verified user token");

        Ok(())
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
        ) -> AppResult<UserToken> {
            Ok(UserToken {
                id: Uuid::new_v4(),
                user_id,
                token,
                expires_at: Some(expires_at),
                created_at: None,
            })
        }

        async fn check_user_token(&self, _user_id: &Uuid) -> AppResult<Option<UserToken>> {
            Ok(None)
        }

        async fn check_validation_status(&self, _user_id: &Uuid) -> AppResult<bool> {
            Ok(false)
        }

        async fn get_user_email(&self, _user_id: &Uuid) -> AppResult<String> {
            Ok(String::new())
        }

        async fn add_verification_email(
            &self,
            _from: &str,
            _to: &str,
            _body: &str,
        ) -> AppResult<()> {
            Ok(())
        }

        async fn verify_user_token(&self, token: &str) -> AppResult<()> {
            assert!(!token.is_empty());
            Ok(())
        }
    }

    struct MockUserTokenEmailService;

    #[async_trait]
    impl UserTokenEmailService for MockUserTokenEmailService {
        async fn send_verification_email(
            &self,
            _to: &[String],
            _token: &str,
        ) -> AppResult<(String, String)> {
            Ok((String::new(), String::new()))
        }
    }

    struct MockUserTokenJwtService;

    impl UserTokenJwtService for MockUserTokenJwtService {
        fn validate_token(&self, _token: &str) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn generate_token_works() {
        let user_token_use_cases = UserTokenUseCases::new(
            Arc::new(MockUserTokenJwtService),
            Arc::new(MockUserTokenEmailService),
            Arc::new(MockUserTokenPersistence),
        );

        let result = user_token_use_cases
            .generate_token_and_send_mail("24d7fa6e-4c52-40ff-ad25-5271e8c48345") // this does not mean the user is in the db, this is just a valid uuid
            .await;

        assert!(result.is_ok());
    }
}
