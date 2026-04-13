use async_trait::async_trait;
use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::{gender::Gender, patient::Patient, sexual_orientation::SexualOrientation},
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
            id: Some(patient_db.id),
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
    async fn create(&self, patient: &Patient) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        if let Some(uid) = patient.user_id {
            let exists =
                sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)", uid)
                    .fetch_one(&self.pool)
                    .await
                    .map_err(AppError::Database)?
                    .unwrap_or(false);

            if !exists {
                return Err(AppError::Internal(String::from("User does not exist")));
            }
        }

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
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

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

    async fn read_single(&self, id: &Uuid) -> AppResult<Patient> {
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

    async fn read_by_user(&self, user_id: &Uuid) -> AppResult<Patient> {
        sqlx::query_as!(
            PatientDb,
            r#"
                SELECT id, user_id, gender_id, sexual_orientation_id, birthdate, phone, emergency_contact_name, emergency_contact_phone, insurance_policy_number, medical_history, current_medications, allergies, created_at
                FROM patients 
                WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(Patient::from)
    }

    async fn read_by_professional(&self, professional_id: &Uuid) -> AppResult<Vec<Patient>> {
        sqlx::query_as!(
            PatientDb,
            r#"
            SELECT DISTINCT p.id, p.user_id, p.gender_id, p.sexual_orientation_id, 
                   p.birthdate, p.phone, p.emergency_contact_name, 
                   p.emergency_contact_phone, p.insurance_policy_number, 
                   p.medical_history, p.current_medications, p.allergies, p.created_at
            FROM patients p
            INNER JOIN sessions s ON p.id = s.patient_id
            WHERE s.professional_id = $1
        "#,
            professional_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|patients| patients.into_iter().map(Patient::from).collect())
    }

    async fn update(&self, patient: &Patient) -> AppResult<()> {
        sqlx::query!(
            "UPDATE patients 
                SET gender_id = $2, sexual_orientation_id = $3, birthdate = $4, phone = $5, emergency_contact_name = $6, emergency_contact_phone = $7, insurance_policy_number = $8, medical_history = $9, current_medications = $10, allergies = $11
                WHERE id = $1",
            patient.id,
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
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }



    async fn update_birthdate(&self, patient_id: &Uuid, birthdate: NaiveDate) -> AppResult<()> {
        sqlx::query!(
            "UPDATE patients SET birthdate = $2 WHERE id = $1",
            patient_id,
            birthdate
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM patients WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }
}
