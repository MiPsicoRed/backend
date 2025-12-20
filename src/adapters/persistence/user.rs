use std::fmt::Display;

use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::user::{Role, User},
    use_cases::user::UserPersistence,
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
            RETURNING id, role_id as role, username, usersurname, email, verified, needs_onboarding, password_hash, created_at",
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
            "SELECT id, role_id as role, username, usersurname, email, verified, needs_onboarding, password_hash, created_at 
            FROM users 
            WHERE email = $1",
            email
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(User::from)
    }

    async fn get_all_users(&self) -> AppResult<Vec<User>> {
        sqlx::query_as!(
            UserDb,
            r#"SELECT id, role_id as role, username, usersurname, email, verified, needs_onboarding, ''::text as "password_hash!", created_at
                FROM users"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|users| users.into_iter().map(User::from).collect())
    }

    async fn user_onboarded(&self, user_id: &Uuid) -> AppResult<()> {
        sqlx::query!(
            r#"
                UPDATE users
                SET needs_onboarding = false
                WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }
}
