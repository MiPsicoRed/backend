use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    app_error::AppResult,
    entities::{gender::Gender, patient::Patient, sexual_orientation::SexualOrientation},
};

#[async_trait]
pub trait PatientPersistence: Send + Sync {
    async fn create(
        &self,
        user_id: Option<Uuid>,
        gender: Gender,
        sexual_orientation: SexualOrientation,
        birthdate: Option<chrono::NaiveDate>,
        phone: &str,
        emergency_contact_name: Option<String>,
        emergency_contact_phone: Option<String>,
        insurance_policy_number: Option<String>,
        medical_history: Option<String>,
        current_medications: Option<String>,
        allergies: Option<String>,
    ) -> AppResult<()>;

    async fn read_all(&self) -> AppResult<Vec<Patient>>;

    async fn read_single(&self, id: Uuid) -> AppResult<Patient>;

    async fn update(
        &self,
        id: Uuid,
        gender: Gender,
        sexual_orientation: SexualOrientation,
        birthdate: Option<chrono::NaiveDate>,
        phone: &str,
        emergency_contact_name: Option<String>,
        emergency_contact_phone: Option<String>,
        insurance_policy_number: Option<String>,
        medical_history: Option<String>,
        current_medications: Option<String>,
        allergies: Option<String>,
    ) -> AppResult<()>;

    async fn delete(&self, id: Uuid) -> AppResult<()>;
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
    pub async fn create(
        &self,
        user_id: Option<Uuid>,
        gender: Gender,
        sexual_orientation: SexualOrientation,
        birthdate: Option<chrono::NaiveDate>,
        phone: &str,
        emergency_contact_name: Option<String>,
        emergency_contact_phone: Option<String>,
        insurance_policy_number: Option<String>,
        medical_history: Option<String>,
        current_medications: Option<String>,
        allergies: Option<String>,
    ) -> AppResult<()> {
        info!("Attempting create patient...");

        self.persistence
            .create(
                user_id,
                gender,
                sexual_orientation,
                birthdate,
                phone,
                emergency_contact_name,
                emergency_contact_phone,
                insurance_policy_number,
                medical_history,
                current_medications,
                allergies,
            )
            .await?;

        info!("Patient created.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn read_all(&self) -> AppResult<Vec<Patient>> {
        self.persistence.read_all().await
    }

    #[instrument(skip(self))]
    pub async fn read_single(&self, id: Uuid) -> AppResult<Patient> {
        self.persistence.read_single(id).await
    }

    #[instrument(skip(self))]
    pub async fn update(
        &self,
        id: Uuid,
        gender: Gender,
        sexual_orientation: SexualOrientation,
        birthdate: Option<chrono::NaiveDate>,
        phone: &str,
        emergency_contact_name: Option<String>,
        emergency_contact_phone: Option<String>,
        insurance_policy_number: Option<String>,
        medical_history: Option<String>,
        current_medications: Option<String>,
        allergies: Option<String>,
    ) -> AppResult<()> {
        info!("Attempting update patient...");

        self.persistence
            .update(
                id,
                gender,
                sexual_orientation,
                birthdate,
                phone,
                emergency_contact_name,
                emergency_contact_phone,
                insurance_policy_number,
                medical_history,
                current_medications,
                allergies,
            )
            .await?;

        info!("Patient updated.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        info!("Attempting delete patient...");

        self.persistence.delete(id).await?;

        info!("Patient deleted.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    #[allow(dead_code)]
    struct MockPatientPersistence;

    #[async_trait]
    impl PatientPersistence for MockPatientPersistence {
        async fn create(
            &self,
            _user_id: Option<Uuid>,
            _gender: Gender,
            _sexual_orientation: SexualOrientation,
            _birthdate: Option<chrono::NaiveDate>,
            _phone: &str,
            _emergency_contact_name: Option<String>,
            _emergency_contact_phone: Option<String>,
            _insurance_policy_number: Option<String>,
            _medical_history: Option<String>,
            _current_medications: Option<String>,
            _allergies: Option<String>,
        ) -> AppResult<()> {
            Ok(())
        }

        async fn read_all(&self) -> AppResult<Vec<Patient>> {
            Ok(vec![])
        }

        async fn read_single(&self, _id: Uuid) -> AppResult<Patient> {
            Ok(Patient {
                id: Uuid::new_v4(),
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

        async fn update(
            &self,
            _id: Uuid,
            _gender: Gender,
            _sexual_orientation: SexualOrientation,
            _birthdate: Option<chrono::NaiveDate>,
            _phone: &str,
            _emergency_contact_name: Option<String>,
            _emergency_contact_phone: Option<String>,
            _insurance_policy_number: Option<String>,
            _medical_history: Option<String>,
            _current_medications: Option<String>,
            _allergies: Option<String>,
        ) -> AppResult<()> {
            Ok(())
        }

        async fn delete(&self, _id: Uuid) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn create_works() {
        let use_cases = PatientUseCases::new(Arc::new(MockPatientPersistence));

        let result = use_cases
            .create(
                None,
                Gender::Male,
                SexualOrientation::Straight,
                None,
                "123456789",
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await;

        assert!(result.is_ok());
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

        let result = use_cases.read_single(Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_works() {
        let use_cases = PatientUseCases::new(Arc::new(MockPatientPersistence));

        let result = use_cases
            .update(
                Uuid::new_v4(),
                Gender::Male,
                SexualOrientation::Straight,
                None,
                "123456789",
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_works() {
        let use_cases = PatientUseCases::new(Arc::new(MockPatientPersistence));

        let result = use_cases.delete(Uuid::new_v4()).await;

        assert!(result.is_ok());
    }
}
