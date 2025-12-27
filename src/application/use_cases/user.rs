use std::sync::Arc;

use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{adapters::crypto::jwt::Claims, app_error::AppResult, entities::user::User};

#[async_trait]
pub trait UserPersistence: Send + Sync {
    async fn create_user_and_patient(
        &self,
        username: &str,
        usersurname: &str,
        email: &str,
        password_hash: &str,
    ) -> AppResult<User>;

    async fn get_user_by_email(&self, email: &str) -> AppResult<User>;

    async fn get_all_users(&self) -> AppResult<Vec<User>>;

    async fn user_onboarded(&self, user_id: &Uuid) -> AppResult<()>;
}

pub trait UserCredentialsHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> AppResult<String>;
    fn verify_password(&self, user_password_hash: &str, input_password: &str) -> AppResult<()>;
}

pub trait UserJwtService: Send + Sync {
    fn generate_token(&self, user: &User) -> AppResult<String>;
    fn validate_token(&self, token: &str) -> AppResult<Claims>;
}

pub trait UserPolarService: Send + Sync {}

#[derive(Clone)]
pub struct UserUseCases {
    pub(crate) jwt_service: Arc<dyn UserJwtService>, // TODO: I had to pub this to access it from the auth middleware, still not sure if this is the okay way to do it.
    polar_service: Arc<dyn UserPolarService>,
    hasher: Arc<dyn UserCredentialsHasher>,
    persistence: Arc<dyn UserPersistence>,
}

impl UserUseCases {
    pub fn new(
        jwt_service: Arc<dyn UserJwtService>,
        polar_service: Arc<dyn UserPolarService>,
        hasher: Arc<dyn UserCredentialsHasher>,
        persistence: Arc<dyn UserPersistence>,
    ) -> Self {
        Self {
            hasher,
            polar_service,
            jwt_service,
            persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn add(
        &self,
        username: &str,
        usersurname: &str,
        email: &str,
        password: &SecretString,
    ) -> AppResult<String> {
        info!("Adding user...");

        let hash = &self.hasher.hash_password(password.expose_secret())?;
        let user = self
            .persistence
            .create_user_and_patient(username, usersurname, email, hash)
            .await?;

        let jwt_token = self.jwt_service.generate_token(&user)?;

        Ok(jwt_token)
    }

    #[instrument(skip(self))]
    pub async fn login(&self, email: &str, password: &SecretString) -> AppResult<String> {
        info!("Attempting user login...");

        let user = self.persistence.get_user_by_email(email).await?;
        self.hasher
            .verify_password(&user.password_hash, password.expose_secret())?;

        info!("User login is valid.");

        let jwt_token = self.jwt_service.generate_token(&user)?;

        Ok(jwt_token)
    }

    #[instrument(skip(self))]
    pub async fn get_all_users(&self) -> AppResult<Vec<User>> {
        info!("Getting all users...");

        let users = self.persistence.get_all_users().await?;

        info!("Got all users.");

        Ok(users)
    }

    #[instrument(skip(self))]
    pub async fn user_onboarded(&self, user_id: &Uuid) -> AppResult<()> {
        info!("Changing user onboarded value...");

        self.persistence.user_onboarded(user_id).await?;

        info!("User onboarded correctly.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use uuid::Uuid;

    use crate::entities::user::Role;

    use super::*;

    struct MockUserPersistence;

    #[async_trait]
    impl UserPersistence for MockUserPersistence {
        async fn create_user_and_patient(
            &self,
            username: &str,
            usersurname: &str,
            email: &str,
            _password_hash: &str,
        ) -> AppResult<User> {
            assert_eq!(username, "john");
            assert_eq!(usersurname, "doe");
            assert_eq!(email, "testuser@gmail.com");

            Ok(User {
                id: Uuid::new_v4(),
                role: Role::default(),
                username: username.to_string(),
                usersurname: usersurname.to_string(),
                email: email.to_string(),
                verified: Some(false),
                needs_onboarding: Some(false),
                password_hash: "".to_string(),
                created_at: None,
            })
        }

        async fn get_user_by_email(&self, email: &str) -> AppResult<User> {
            assert_eq!(email, "testuser@gmail.com");
            Ok(User {
                id: Uuid::new_v4(),
                role: Role::default(),
                username: "john".to_string(),
                usersurname: "doe".to_string(),
                email: email.to_string(),
                verified: Some(false),
                needs_onboarding: Some(false),
                password_hash: "hashed_password".to_string(),
                created_at: None,
            })
        }

        async fn get_all_users(&self) -> AppResult<Vec<User>> {
            Ok(vec![User {
                id: Uuid::new_v4(),
                role: Role::default(),
                username: "john".to_string(),
                usersurname: "doe".to_string(),
                email: "testuser@gmail.com".to_string(),
                verified: Some(false),
                needs_onboarding: Some(false),
                password_hash: "hashed_password".to_string(),
                created_at: None,
            }])
        }

        async fn user_onboarded(&self, _user_id: &Uuid) -> AppResult<()> {
            Ok(())
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
        fn generate_token(&self, user: &User) -> AppResult<String> {
            Ok(format!("token_{}", user.username))
        }

        fn validate_token(&self, token: &str) -> AppResult<Claims> {
            if token.starts_with("token_") {
                Ok(Claims::default())
            } else {
                Err(crate::app_error::AppError::Unauthorized(
                    "Invalid Token".into(),
                ))
            }
        }
    }

    struct MockUserPolarService;

    impl UserPolarService for MockUserPolarService {}

    #[tokio::test]
    async fn add_user_works() {
        let user_use_cases = UserUseCases::new(
            Arc::new(MockUserJWTService),
            Arc::new(MockUserPolarService),
            Arc::new(MockUserCredentialsHasher),
            Arc::new(MockUserPersistence),
        );

        let result = user_use_cases
            .add("john", "doe", "testuser@gmail.com", &"testuser_pw".into())
            .await;

        assert!(result.is_ok());
    }
}
