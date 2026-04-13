use std::sync::Arc;

use async_trait::async_trait;

use tracing::{info, instrument};
use uuid::Uuid;
use chrono::NaiveDate;

use crate::{app_error::AppResult, entities::patient::Patient};

#[async_trait]
pub trait PatientPersistence: Send + Sync {
    async fn create(&self, patient: &Patient) -> AppResult<()>;

    async fn read_all(&self) -> AppResult<Vec<Patient>>;

    async fn read_single(&self, id: &Uuid) -> AppResult<Patient>;

    async fn read_by_user(&self, user_id: &Uuid) -> AppResult<Patient>;

    async fn read_by_professional(&self, professional_id: &Uuid) -> AppResult<Vec<Patient>>;

    async fn update(&self, patient: &Patient) -> AppResult<()>;

    async fn update_birthdate(&self, patient_id: &Uuid, birthdate: NaiveDate) -> AppResult<()>;

    async fn delete(&self, id: &Uuid) -> AppResult<()>;
}

#[derive(Clone)]
pub struct PatientUseCases {
    persistence: Arc<dyn PatientPersistence>,
}

impl PatientUseCases {
    pub fn new(persistence: Arc<dyn PatientPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn create(&self, patient: &Patient) -> AppResult<()> {
        info!("Attempting create patient...");

        self.persistence.create(patient).await?;

        info!("Patient created.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn read_all(&self) -> AppResult<Vec<Patient>> {
        self.persistence.read_all().await
    }

    #[instrument(skip(self))]
    pub async fn read_single(&self, id: &Uuid) -> AppResult<Patient> {
        self.persistence.read_single(id).await
    }

    #[instrument(skip(self))]
    pub async fn read_by_user(&self, user_id: &Uuid) -> AppResult<Patient> {
        self.persistence.read_by_user(user_id).await
    }

    #[instrument(skip(self))]
    pub async fn read_by_professional(&self, professional_id: &Uuid) -> AppResult<Vec<Patient>> {
        self.persistence.read_by_professional(professional_id).await
    }

    #[instrument(skip(self))]
    pub async fn update(&self, patient: &Patient) -> AppResult<()> {
        info!("Attempting update patient...");

        self.persistence.update(patient).await?;

        info!("Patient updated.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: &Uuid) -> AppResult<()> {
        info!("Attempting delete patient...");

        self.persistence.delete(id).await?;

        info!("Patient deleted.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use crate::{
        app_error::AppError,
        entities::{gender::Gender, sexual_orientation::SexualOrientation},
    };

    use super::*;

    #[allow(dead_code)]
    struct MockPatientPersistence;

    #[async_trait]
    impl PatientPersistence for MockPatientPersistence {
        async fn create(&self, patient: &Patient) -> AppResult<()> {
            if patient.id.is_some() {
                return Err(AppError::Internal(
                    "patient id must be None when creating".into(),
                ));
            }

            Ok(())
        }

        async fn read_all(&self) -> AppResult<Vec<Patient>> {
            Ok(vec![])
        }

        async fn read_single(&self, _id: &Uuid) -> AppResult<Patient> {
            Ok(Patient {
                id: Some(Uuid::new_v4()),
                user_id: None,
                gender: Gender::Male,
                sexual_orientation: SexualOrientation::Straight,
                birthdate: None,
                phone: "123456789".to_string(),
                emergency_contact_name: None,
                emergency_contact_phone: None,
                insurance_policy_number: None,
                medical_history: None,
                current_medications: None,
                allergies: None,
                created_at: Some(chrono::Utc::now().naive_utc()),
            })
        }

        async fn read_by_user(&self, _user_id: &Uuid) -> AppResult<Patient> {
            Ok(Patient {
                id: Some(Uuid::new_v4()),
                user_id: None,
                gender: Gender::Male,
                sexual_orientation: SexualOrientation::Straight,
                birthdate: None,
                phone: "123456789".to_string(),
                emergency_contact_name: None,
                emergency_contact_phone: None,
                insurance_policy_number: None,
                medical_history: None,
                current_medications: None,
                allergies: None,
                created_at: Some(chrono::Utc::now().naive_utc()),
            })
        }

        async fn read_by_professional(&self, _professional_id: &Uuid) -> AppResult<Vec<Patient>> {
            Ok(vec![])
        }

        async fn update(&self, patient: &Patient) -> AppResult<()> {
            assert!(patient.id.is_some());

             Ok(())
        }

        async fn update_birthdate(&self, _patient_id: &Uuid, _birthdate: NaiveDate) -> AppResult<()> {
            Ok(())
        }

        async fn delete(&self, _id: &Uuid) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn create_works() {
        let use_cases = PatientUseCases::new(Arc::new(MockPatientPersistence));

        let result = use_cases
            .create(&Patient {
                id: None,
                user_id: None,
                gender: Gender::Male,
                sexual_orientation: SexualOrientation::Straight,
                birthdate: None,
                phone: "123456789".to_string(),
                emergency_contact_name: None,
                emergency_contact_phone: None,
                insurance_policy_number: None,
                medical_history: None,
                current_medications: None,
                allergies: None,
                created_at: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn create_with_id_fails() {
        let use_cases = PatientUseCases::new(Arc::new(MockPatientPersistence));

        let result = use_cases
            .create(&Patient {
                id: Some(Uuid::new_v4()),
                user_id: None,
                gender: Gender::Male,
                sexual_orientation: SexualOrientation::Straight,
                birthdate: None,
                phone: "123456789".to_string(),
                emergency_contact_name: None,
                emergency_contact_phone: None,
                insurance_policy_number: None,
                medical_history: None,
                current_medications: None,
                allergies: None,
                created_at: None,
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn read_all_works() {
        let use_cases = PatientUseCases::new(Arc::new(MockPatientPersistence));

        let result = use_cases.read_all().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_single_works() {
        let use_cases = PatientUseCases::new(Arc::new(MockPatientPersistence));

        let result = use_cases.read_single(&Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_user_works() {
        let use_cases = PatientUseCases::new(Arc::new(MockPatientPersistence));

        let result = use_cases.read_by_user(&Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_by_professional_works() {
        let use_cases = PatientUseCases::new(Arc::new(MockPatientPersistence));

        let result = use_cases.read_by_professional(&Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_works() {
        let use_cases = PatientUseCases::new(Arc::new(MockPatientPersistence));

        let result = use_cases
            .update(&Patient {
                id: Some(Uuid::new_v4()),
                user_id: None,
                gender: Gender::Male,
                sexual_orientation: SexualOrientation::Straight,
                birthdate: None,
                phone: "123456789".to_string(),
                emergency_contact_name: None,
                emergency_contact_phone: None,
                insurance_policy_number: None,
                medical_history: None,
                current_medications: None,
                allergies: None,
                created_at: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_works() {
        let use_cases = PatientUseCases::new(Arc::new(MockPatientPersistence));

        let result = use_cases.delete(&Uuid::new_v4()).await;

        assert!(result.is_ok());
    }
}
