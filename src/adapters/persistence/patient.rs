use async_trait::async_trait;
use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::{PostgresPersistence, user::UserDb},
    app_error::{AppError, AppResult},
    entities::{
        gender::Gender, patient::Patient, sexual_orientation::SexualOrientation, user::User,
    },
    use_cases::patient::PatientPersistence,
};

// Patient struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct PatientDb {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub gender_id: i32,
    pub sexual_orientation_id: i32,
    pub birthdate: Option<NaiveDate>,
    pub phone: String,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub insurance_policy_number: Option<String>,
    pub medical_history: Option<String>,
    pub current_medications: Option<String>,
    pub allergies: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<PatientDb> for Patient {
    fn from(patient_db: PatientDb) -> Self {
        Patient {
            id: patient_db.id,
            user_id: patient_db.user_id,
            gender: Gender::from_id(patient_db.gender_id).unwrap_or_default(),
            sexual_orientation: SexualOrientation::from_id(patient_db.sexual_orientation_id)
                .unwrap_or_default(),
            birthdate: patient_db.birthdate,
            phone: patient_db.phone,
            emergency_contact_name: patient_db.emergency_contact_name,
            emergency_contact_phone: patient_db.emergency_contact_phone,
            insurance_policy_number: patient_db.insurance_policy_number,
            medical_history: patient_db.medical_history,
            current_medications: patient_db.current_medications,
            allergies: patient_db.allergies,
            created_at: patient_db.created_at,
        }
    }
}

#[async_trait]
impl PatientPersistence for PostgresPersistence {
    async fn create(
        &self,
        user_id: Option<Uuid>,
        gender: Gender,
        sexual_orientation: SexualOrientation,
        birthdate: Option<NaiveDate>,
        phone: Option<String>,
        emergency_contact_name: Option<String>,
        emergency_contact_phone: Option<String>,
        insurance_policy_number: Option<String>,
        medical_history: Option<String>,
        current_medications: Option<String>,
        allergies: Option<String>,
    ) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        // If we are creating a patient from an already existing user we already have the birthdate and the phone
        if user_id.is_some() {
            let user = sqlx::query_as!(
            UserDb,
                "SELECT id, role_id as role, username, usersurname, email, phone, birthdate, verified, password_hash, created_at 
                FROM users 
                WHERE id = $1",
                uuid
            )
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::Database)
            .map(User::from)?;

            sqlx::query!(
                    "INSERT INTO patients (id, user_id, gender_id, sexual_orientation_id, birthdate, phone, emergency_contact_name, emergency_contact_phone, insurance_policy_number, medical_history, current_medications, allergies) 
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
                    uuid,
                    user_id,
                    gender.to_id(),
                    sexual_orientation.to_id(),
                    user.birthdate,
                    user.phone,
                    emergency_contact_name,
                    emergency_contact_phone,
                    insurance_policy_number,
                    medical_history,
                    current_medications,
                    allergies
                )
                .execute(&self.pool)
                .await
                .map_err(AppError::Database)?;
        } else {
            sqlx::query!(
                "INSERT INTO patients (id, user_id, gender_id, sexual_orientation_id, birthdate, phone, emergency_contact_name, emergency_contact_phone, insurance_policy_number, medical_history, current_medications, allergies) 
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
                uuid,
                user_id,
                gender.to_id(),
                sexual_orientation.to_id(),
                birthdate,
                phone,
                emergency_contact_name,
                emergency_contact_phone,
                insurance_policy_number,
                medical_history,
                current_medications,
                allergies
            )
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        }

        Ok(())
    }

    async fn read_all(&self) -> AppResult<Vec<Patient>> {
        sqlx::query_as!(
            PatientDb,
            r#"
                SELECT id, user_id, gender_id, sexual_orientation_id, birthdate, phone, emergency_contact_name, emergency_contact_phone, insurance_policy_number, medical_history, current_medications, allergies, created_at
                FROM patients
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|patients| patients.into_iter().map(Patient::from).collect())
    }

    async fn read_single(&self, id: Uuid) -> AppResult<Patient> {
        sqlx::query_as!(
            PatientDb,
            r#"
                SELECT id, user_id, gender_id, sexual_orientation_id, birthdate, phone, emergency_contact_name, emergency_contact_phone, insurance_policy_number, medical_history, current_medications, allergies, created_at
                FROM patients 
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(Patient::from)
    }

    async fn update(
        &self,
        id: Uuid,
        gender: Gender,
        sexual_orientation: SexualOrientation,
        birthdate: Option<NaiveDate>,
        phone: Option<String>,
        emergency_contact_name: Option<String>,
        emergency_contact_phone: Option<String>,
        insurance_policy_number: Option<String>,
        medical_history: Option<String>,
        current_medications: Option<String>,
        allergies: Option<String>,
    ) -> AppResult<()> {
        sqlx::query!(
            "UPDATE patients 
                SET gender_id = $2, sexual_orientation_id = $3, birthdate = $4, phone = $5, emergency_contact_name = $6, emergency_contact_phone = $7, insurance_policy_number = $8, medical_history = $9, current_medications = $10, allergies = $11
                WHERE id = $1",
            id,
            gender.to_id(),
            sexual_orientation.to_id(),
            birthdate,
            phone,
            emergency_contact_name,
            emergency_contact_phone,
            insurance_policy_number,
            medical_history,
            current_medications,
            allergies
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM patients WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }
}
