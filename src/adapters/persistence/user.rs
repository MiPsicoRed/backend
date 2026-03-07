use std::fmt::Display;

use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::user::{Role, User},
    use_cases::user::{UserPersistence, OnboardingDto},
};

// User struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UserDb {
    pub id: Uuid,
    pub role: RoleDb,
    pub username: String,
    pub usersurname: String,
    pub email: String,
    pub verified: Option<bool>,
    pub needs_onboarding: Option<bool>,
    pub password_hash: String,
    pub profile_picture_url: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<UserDb> for User {
    fn from(user_db: UserDb) -> Self {
        User {
            id: user_db.id,
            role: user_db.role.into(),
            username: user_db.username,
            usersurname: user_db.usersurname,
            email: user_db.email,
            verified: user_db.verified,
            needs_onboarding: user_db.needs_onboarding,
            password_hash: user_db.password_hash,
            profile_picture_url: user_db.profile_picture_url,
            created_at: user_db.created_at,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum RoleDb {
    #[default]
    Patient,
    Professional,
    Admin,
}

impl From<RoleDb> for Role {
    fn from(value: RoleDb) -> Self {
        match value {
            RoleDb::Patient => Role::Patient,
            RoleDb::Professional => Role::Professional,
            RoleDb::Admin => Role::Admin,
        }
    }
}

impl Display for RoleDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Role::from(*self).fmt(f)
    }
}

impl RoleDb {
    pub fn to_id(self) -> i32 {
        Role::from(self).to_id()
    }

    pub fn from_id(id: i32) -> Option<Self> {
        Role::from_id(id).map(|kind| match kind {
            Role::Patient => RoleDb::Patient,
            Role::Professional => RoleDb::Professional,
            Role::Admin => RoleDb::Admin,
        })
    }
}

impl From<i32> for RoleDb {
    fn from(id: i32) -> Self {
        RoleDb::from_id(id).unwrap_or_default()
    }
}

#[async_trait]
impl UserPersistence for PostgresPersistence {
    async fn create_user_and_patient(
        &self,
        username: &str,
        usersurname: &str,
        email: &str,
        password_hash: &str,
    ) -> AppResult<User> {
        use crate::entities::patient::Patient;
        let uuid = Uuid::new_v4();

        let mut tx = self.pool.begin().await.map_err(AppError::Database)?;

        let user = sqlx::query_as!(
            UserDb,
            "INSERT INTO users (id, role_id, username, usersurname, email, password_hash) 
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, role_id as role, username, usersurname, email, verified, needs_onboarding, password_hash, profile_picture_url, created_at",
            uuid,
            RoleDb::default().to_id(),
            username,
            usersurname,
            email,
            password_hash
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(AppError::Database)
        .map(User::from)?;

        let patient = Patient::from(&user);

        sqlx::query!(
        "INSERT INTO patients (id, user_id, gender_id, sexual_orientation_id, birthdate, phone, emergency_contact_name, emergency_contact_phone, insurance_policy_number, medical_history, current_medications, allergies) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            uuid,
            patient.user_id,
            patient.gender.to_id(),
            patient.sexual_orientation.to_id(),
            patient.birthdate,
            patient.phone,
            patient.emergency_contact_name,
            patient.emergency_contact_phone,
            patient.insurance_policy_number,
            patient.medical_history,
            patient.current_medications,
            patient.allergies
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        tx.commit().await.map_err(AppError::Database)?;

        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> AppResult<User> {
        sqlx::query_as!(
            UserDb,
            "SELECT id, role_id as role, username, usersurname, email, verified, needs_onboarding, password_hash, profile_picture_url, created_at 
            FROM users 
            WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(User::from)
    }

    async fn get_user_by_id(&self, user_id: &Uuid) -> AppResult<User> {
        sqlx::query_as!(
            UserDb,
            "SELECT id, role_id as role, username, usersurname, email, verified, needs_onboarding, password_hash, profile_picture_url, created_at 
            FROM users 
            WHERE id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))
        .map(User::from)
    }

    async fn get_all_users(&self) -> AppResult<Vec<User>> {
        sqlx::query_as!(
            UserDb,
            r#"SELECT id, role_id as role, username, usersurname, email, verified, needs_onboarding, ''::text as "password_hash!", profile_picture_url, created_at
                FROM users"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|users| users.into_iter().map(User::from).collect())
    }

    async fn get_onboarding_info(&self, user_id: &Uuid) -> AppResult<Option<OnboardingDto>> {
        let onboarding = sqlx::query!(
            r#"
                SELECT user_type, full_name, phone, birthdate, reason, experience
                FROM user_onboardings
                WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        if let Some(ob) = onboarding {
            let consent = sqlx::query!(
                r#"
                    SELECT is_monoparental, guardian_name, guardian_id_document, signature, guardian2_name, guardian2_id_document, signature2
                    FROM user_consents
                    WHERE user_id = $1
                "#,
                user_id
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::Database)?;

            Ok(Some(OnboardingDto {
                user_id: *user_id,
                user_type: ob.user_type,
                full_name: ob.full_name,
                phone: ob.phone,
                birthdate: ob.birthdate,
                reason: ob.reason,
                experience: ob.experience,
                is_monoparental: consent.as_ref().map(|c| c.is_monoparental).unwrap_or(true),
                guardian_name: consent.as_ref().and_then(|c| c.guardian_name.clone()),
                guardian_id_document: consent.as_ref().and_then(|c| c.guardian_id_document.clone()),
                signature: consent.as_ref().and_then(|c| c.signature.clone()),
                guardian2_name: consent.as_ref().and_then(|c| c.guardian2_name.clone()),
                guardian2_id_document: consent.as_ref().and_then(|c| c.guardian2_id_document.clone()),
                signature2: consent.as_ref().and_then(|c| c.signature2.clone()),
            }))
        } else {
            Ok(None)
        }
    }

    async fn user_onboarded(&self, dto: OnboardingDto) -> AppResult<()> {
        let mut tx = self.pool.begin().await.map_err(AppError::Database)?;

        sqlx::query!(
            r#"
                UPDATE users
                SET needs_onboarding = false
                WHERE id = $1
            "#,
            dto.user_id
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        // Ensure we wipe old onboardings/consents before saving to execute a pseudo-upsert
        sqlx::query!("DELETE FROM user_onboardings WHERE user_id = $1", dto.user_id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        sqlx::query!("DELETE FROM user_consents WHERE user_id = $1", dto.user_id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

        let onboarding_id = Uuid::new_v4();
        sqlx::query!(
            r#"
                INSERT INTO user_onboardings (id, user_id, user_type, full_name, phone, birthdate, reason, experience)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            onboarding_id,
            dto.user_id,
            dto.user_type,
            dto.full_name,
            dto.phone,
            dto.birthdate,
            dto.reason,
            dto.experience
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        let consent_id = Uuid::new_v4();
        sqlx::query!(
            r#"
                INSERT INTO user_consents (id, user_id, is_monoparental, guardian_name, guardian_id_document, signature, guardian2_name, guardian2_id_document, signature2)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            consent_id,
            dto.user_id,
            dto.is_monoparental,
            dto.guardian_name,
            dto.guardian_id_document,
            dto.signature,
            dto.guardian2_name,
            dto.guardian2_id_document,
            dto.signature2
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        tx.commit().await.map_err(AppError::Database)?;

        Ok(())
    }

    async fn update_profile_picture_url(
        &self,
        user_id: &Uuid,
        profile_picture_url: &str,
    ) -> AppResult<()> {
        sqlx::query!(
            r#"
                UPDATE users
                SET profile_picture_url = $1
                WHERE id = $2
            "#,
            profile_picture_url,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }
}
