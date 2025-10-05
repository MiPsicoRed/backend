use std::sync::Arc;

use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use tracing::{info, instrument};

use crate::{adapters::persistence::user::UserDb, app_error::AppResult};

#[async_trait]
pub trait UserPersistence: Send + Sync {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()>;
    async fn get_user_by_username(&self, username: &str) -> AppResult<UserDb>;
    async fn get_all_users(&self) -> AppResult<Vec<UserDb>>;
}

pub trait UserCredentialsHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> AppResult<String>;
    fn verify_password(&self, user_password_hash: &str, input_password: &str) -> AppResult<()>;
}

pub trait UserJwtService: Send + Sync {
    fn generate_token(&self, user: &UserDb) -> AppResult<String>;
    fn validate_token(&self, token: &str) -> AppResult<()>;
}

#[derive(Clone)]
pub struct UserUseCases {
    pub(crate) jwt_service: Arc<dyn UserJwtService>, // TODO: I had to pub this to access it from the auth middleware, still not sure if this is the okay way to do it.
    hasher: Arc<dyn UserCredentialsHasher>,
    persistence: Arc<dyn UserPersistence>,
}

impl UserUseCases {
    pub fn new(
        jwt_service: Arc<dyn UserJwtService>,
        hasher: Arc<dyn UserCredentialsHasher>,
        persistence: Arc<dyn UserPersistence>,
    ) -> Self {
        Self {
            hasher,
            jwt_service,
            persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn add(&self, username: &str, email: &str, password: &SecretString) -> AppResult<()> {
        info!("Adding user...");

        let hash = &self.hasher.hash_password(password.expose_secret())?;
        self.persistence.create_user(username, email, hash).await?;

        info!("Adding user finished.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn login(&self, username: &str, password: &SecretString) -> AppResult<String> {
        info!("Attempting user login...");

        let user = self.persistence.get_user_by_username(username).await?;
        self.hasher
            .verify_password(&user.password_hash, password.expose_secret())?;

        info!("User login is valid.");

        let jwt_token = self.jwt_service.generate_token(&user)?;

        Ok(jwt_token)
    }

    #[instrument(skip(self))]
    pub async fn get_all_users(&self) -> AppResult<Vec<UserDb>> {
        info!("Getting all users...");

        let users = self.persistence.get_all_users().await?;

        info!("Got all users.");

        Ok(users)
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use uuid::Uuid;

    use super::*;

    struct MockUserPersistence;

    #[async_trait]
    impl UserPersistence for MockUserPersistence {
        async fn create_user(
            &self,
            username: &str,
            email: &str,
            _password_hash: &str,
        ) -> AppResult<()> {
            assert_eq!(username, "testuser");
            assert_eq!(email, "testuser@gmail.com");

            Ok(())
        }

        async fn get_user_by_username(&self, username: &str) -> AppResult<UserDb> {
            assert_eq!(username, "testuser");
            Ok(UserDb {
                id: Uuid::new_v4(),
                username: username.to_string(),
                email: "testuser@gmail.com".to_string(),
                password_hash: "hashed_password".to_string(),
                created_at: None,
            })
        }

        async fn get_all_users(&self) -> AppResult<Vec<UserDb>> {
            Ok(vec![UserDb {
                id: Uuid::new_v4(),
                username: "testuser".to_string(),
                email: "testuser@gmail.com".to_string(),
                password_hash: "hashed_password".to_string(),
                created_at: None,
            }])
        }
    }

    struct MockUserCredentialsHasher;

    impl UserCredentialsHasher for MockUserCredentialsHasher {
        fn hash_password(&self, password: &str) -> AppResult<String> {
            Ok(format!("{}_hash", password))
        }

        fn verify_password(&self, user_password_hash: &str, input_password: &str) -> AppResult<()> {
            let expected_hash = format!("{}_hash", input_password);

            if user_password_hash == expected_hash {
                Ok(())
            } else {
                Err(crate::app_error::AppError::InvalidCredentials)
            }
        }
    }

    struct MockUserJWTService;

    impl UserJwtService for MockUserJWTService {
        fn generate_token(&self, user: &UserDb) -> AppResult<String> {
            Ok(format!("token_{}", user.username))
        }

        fn validate_token(&self, token: &str) -> AppResult<()> {
            if token.starts_with("token_") {
                Ok(())
            } else {
                Err(crate::app_error::AppError::Unauthorized(
                    "Invalid Token".into(),
                ))
            }
        }
    }

    #[tokio::test]
    async fn add_user_works() {
        let user_use_cases = UserUseCases::new(
            Arc::new(MockUserJWTService),
            Arc::new(MockUserCredentialsHasher),
            Arc::new(MockUserPersistence),
        );

        let result = user_use_cases
            .add("testuser", "testuser@gmail.com", &"testuser_pw".into())
            .await;

        assert!(result.is_ok());
    }
}
