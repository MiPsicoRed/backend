use std::sync::Arc;

use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{adapters::crypto::jwt::Claims, app_error::AppResult, entities::user::User};

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct OnboardingDto {
    pub user_id: Uuid,
    pub user_type: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    #[schema(value_type = Option<String>)]
    pub birthdate: Option<chrono::NaiveDate>,
    pub reason: Option<String>,
    pub experience: Option<String>,
    pub is_monoparental: bool,
    pub guardian_name: Option<String>,
    pub guardian_id_document: Option<String>,
    pub signature: Option<String>,
    pub guardian2_name: Option<String>,
    pub guardian2_id_document: Option<String>,
    pub signature2: Option<String>,
}

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

    async fn get_user_by_id(&self, user_id: &Uuid) -> AppResult<User>;

    async fn get_all_users(&self) -> AppResult<Vec<User>>;

    async fn get_onboarding_info(&self, user_id: &Uuid) -> AppResult<Option<OnboardingDto>>;

    async fn user_onboarded(&self, dto: OnboardingDto) -> AppResult<()>;

    async fn update_profile_picture_url(
        &self,
        user_id: &Uuid,
        profile_picture_url: &str,
    ) -> AppResult<()>;
}

pub trait UserCredentialsHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> AppResult<String>;
    fn verify_password(&self, user_password_hash: &str, input_password: &str) -> AppResult<()>;
}

pub trait UserJwtService: Send + Sync {
    fn generate_token(&self, user: &User) -> AppResult<String>;
    fn validate_token(&self, token: &str) -> AppResult<Claims>;
}

#[derive(Clone)]
pub struct UserUseCases {
    pub(crate) jwt_service: Arc<dyn UserJwtService>, // TODO: I had to pub this to access it from the auth middleware, still not sure if this is the okay way to do it.
    hasher: Arc<dyn UserCredentialsHasher>,
    persistence: Arc<dyn UserPersistence>,
    #[allow(dead_code)]
    patient_persistence: Arc<dyn crate::use_cases::patient::PatientPersistence>,
    #[allow(dead_code)]
    parent_consent_persistence: Arc<dyn crate::use_cases::parent_consent::ParentConsentPersistence>,
}

impl UserUseCases {
    pub fn new(
        jwt_service: Arc<dyn UserJwtService>,
        hasher: Arc<dyn UserCredentialsHasher>,
        persistence: Arc<dyn UserPersistence>,
        patient_persistence: Arc<dyn crate::use_cases::patient::PatientPersistence>,
        parent_consent_persistence: Arc<dyn crate::use_cases::parent_consent::ParentConsentPersistence>,
    ) -> Self {
        Self {
            hasher,
            jwt_service,
            persistence,
            patient_persistence,
            parent_consent_persistence,
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


    pub async fn get_user_by_id(&self, user_id: &Uuid) -> AppResult<User> {
        info!("Getting user by id...");

        let user = self.persistence.get_user_by_id(user_id).await?;

        info!("Got user by id.");

        Ok(user)
    }

    #[instrument(skip(self))]
    pub async fn get_onboarding_info(&self, user_id: &Uuid) -> AppResult<Option<OnboardingDto>> {
        info!("Getting user onboarding info...");
        let info = self.persistence.get_onboarding_info(user_id).await?;
        Ok(info)
    }

    #[instrument(skip(self))]
    pub async fn user_onboarded(&self, dto: OnboardingDto) -> AppResult<()> {
        info!("Changing user onboarded value and recording consent details...");

        self.persistence.user_onboarded(dto).await?;

        info!("User onboarded correctly.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn update_profile_picture_url(
        &self,
        user_id: &Uuid,
        profile_picture_url: &str,
    ) -> AppResult<()> {
        info!("Updating user profile picture url...");

        self.persistence
            .update_profile_picture_url(user_id, profile_picture_url)
            .await?;

        info!("User profile picture url updated correctly.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use chrono::NaiveDate;
    use uuid::Uuid;

    use crate::entities::user::Role;
    use crate::domain::entities::parent_consent::ParentConsent;

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
                profile_picture_url: None,
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
                profile_picture_url: None,
                created_at: None,
            })
        }

        async fn get_user_by_id(&self, user_id: &Uuid) -> AppResult<User> {
            Ok(User {
                id: user_id.clone(),
                role: Role::default(),
                username: "john".to_string(),
                usersurname: "doe".to_string(),
                email: "testuser@gmail.com".to_string(),
                verified: Some(false),
                needs_onboarding: Some(false),
                password_hash: "hashed_password".to_string(),
                profile_picture_url: None,
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
                profile_picture_url: None,
                created_at: None,
            }])
        }

        async fn user_onboarded(&self, _dto: OnboardingDto) -> AppResult<()> {
            Ok(())
        }

        async fn get_onboarding_info(&self, _user_id: &Uuid) -> AppResult<Option<OnboardingDto>> {
            Ok(None)
        }

        async fn update_profile_picture_url(
            &self,
            _user_id: &Uuid,
            _profile_picture_url: &str,
        ) -> AppResult<()> {
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

    struct MockPatientPersistence;

    #[async_trait]
    impl crate::use_cases::patient::PatientPersistence for MockPatientPersistence {
        async fn create(&self, _patient: &crate::entities::patient::Patient) -> AppResult<()> { Ok(()) }
        async fn read_all(&self) -> AppResult<Vec<crate::entities::patient::Patient>> { Ok(vec![]) }
        async fn read_single(&self, _id: &Uuid) -> AppResult<crate::entities::patient::Patient> { Err(crate::app_error::AppError::Internal("Not impl".into())) }
        async fn read_by_user(&self, _user_id: &Uuid) -> AppResult<crate::entities::patient::Patient> { 
             Ok(crate::entities::patient::Patient {
                id: Some(Uuid::new_v4()),
                user_id: None,
                gender: crate::entities::gender::Gender::default(),
                sexual_orientation: crate::entities::sexual_orientation::SexualOrientation::default(),
                birthdate: None,
                phone: "".to_string(),
                emergency_contact_name: None,
                emergency_contact_phone: None,
                insurance_policy_number: None,
                medical_history: None,
                current_medications: None,
                allergies: None,
                created_at: None,
            })
        }
        async fn read_by_professional(&self, _professional_id: &Uuid) -> AppResult<Vec<crate::entities::patient::Patient>> { Ok(vec![]) }
        async fn update(&self, _patient: &crate::entities::patient::Patient) -> AppResult<()> { Ok(()) }
        async fn update_birthdate(&self, _patient_id: &Uuid, _birthdate: NaiveDate) -> AppResult<()> { Ok(()) }
        async fn delete(&self, _id: &Uuid) -> AppResult<()> { Ok(()) }
    }

    struct MockParentConsentPersistence;

    #[async_trait]
    impl crate::use_cases::parent_consent::ParentConsentPersistence for MockParentConsentPersistence {
        async fn create(&self, _consent: &ParentConsent) -> AppResult<()> { Ok(()) }
        async fn read_by_patient(&self, _patient_id: &Uuid) -> AppResult<Option<ParentConsent>> { Ok(None) }
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

    #[tokio::test]
    async fn add_user_works() {
        let user_use_cases = UserUseCases::new(
            Arc::new(MockUserJWTService),
            Arc::new(MockUserCredentialsHasher),
            Arc::new(MockUserPersistence),
            Arc::new(MockPatientPersistence),
            Arc::new(MockParentConsentPersistence),
        );

        let result = user_use_cases
            .add("john", "doe", "testuser@gmail.com", &"testuser_pw".into())
            .await;

        assert!(result.is_ok());
    }
}
