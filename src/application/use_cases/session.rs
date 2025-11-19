use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{app_error::AppResult, entities::session::Session};

#[async_trait]
pub trait SessionPersistence: Send + Sync {
    async fn create(&self, session: &Session) -> AppResult<()>;

    async fn read_all(&self) -> AppResult<Vec<Session>>;

    async fn read_patient(&self, patient_id: &Uuid) -> AppResult<Vec<Session>>;

    async fn read_professional(&self, professional_id: &Uuid) -> AppResult<Vec<Session>>;

    async fn read_single(&self, id: &Uuid) -> AppResult<Session>;

    async fn update(&self, session: &Session) -> AppResult<()>;

    async fn delete(&self, id: &Uuid) -> AppResult<()>;
}

#[derive(Clone)]
pub struct SessionUseCases {
    persistence: Arc<dyn SessionPersistence>,
}

impl SessionUseCases {
    pub fn new(persistence: Arc<dyn SessionPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn create(&self, session: &Session) -> AppResult<()> {
        info!("Attempting create session...");

        self.persistence.create(session).await?;

        info!("Session created.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn read_all(&self) -> AppResult<Vec<Session>> {
        self.persistence.read_all().await
    }

    #[instrument(skip(self))]
    pub async fn read_patient(&self, patient_id: &Uuid) -> AppResult<Vec<Session>> {
        self.persistence.read_patient(patient_id).await
    }

    #[instrument(skip(self))]
    pub async fn read_professional(&self, professional_id: &Uuid) -> AppResult<Vec<Session>> {
        self.persistence.read_professional(professional_id).await
    }

    #[instrument(skip(self))]
    pub async fn read_single(&self, id: &Uuid) -> AppResult<Session> {
        self.persistence.read_single(id).await
    }

    #[instrument(skip(self))]
    pub async fn update(&self, session: &Session) -> AppResult<()> {
        info!("Attempting update session...");

        self.persistence.update(session).await?;

        info!("Sessión updated.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: &Uuid) -> AppResult<()> {
        info!("Attempting delete session...");

        self.persistence.delete(id).await?;

        info!("Session deleted.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use crate::{app_error::AppError, entities::session::SessionStatus};

    use super::*;

    struct MockSessionPersistence;

    #[async_trait]
    impl SessionPersistence for MockSessionPersistence {
        async fn create(&self, session: &Session) -> AppResult<()> {
            if session.id.is_some() {
                return Err(AppError::Internal(
                    "session id must be None when creating".into(),
                ));
            }

            Ok(())
        }

        async fn read_all(&self) -> AppResult<Vec<Session>> {
            Ok(vec![])
        }

        async fn read_patient(&self, _patient_id: &Uuid) -> AppResult<Vec<Session>> {
            Ok(vec![])
        }

        async fn read_professional(&self, _professional_id: &Uuid) -> AppResult<Vec<Session>> {
            Ok(vec![])
        }

        async fn read_single(&self, _id: &Uuid) -> AppResult<Session> {
            Ok(Session {
                id: Some(Uuid::new_v4()),
                patient_id: Uuid::new_v4(),
                professional_id: Uuid::new_v4(),
                session_type_id: Some(Uuid::new_v4()),
                session_status: SessionStatus::Scheduled,
                session_date: None,
                videocall_url: Some(String::from("https://videocallurl.com")),
                notes: Some(String::from("")),
                session_duration: Some(30),
                completed: false,
                created_at: None,
            })
        }

        async fn update(&self, session: &Session) -> AppResult<()> {
            assert!(session.id.is_some());

            Ok(())
        }

        async fn delete(&self, _id: &Uuid) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn create_works() {
        let use_cases = SessionUseCases::new(Arc::new(MockSessionPersistence));

        let result = use_cases
            .create(&Session {
                id: None,
                patient_id: Uuid::new_v4(),
                professional_id: Uuid::new_v4(),
                session_type_id: Some(Uuid::new_v4()),
                session_status: SessionStatus::Scheduled,
                session_date: None,
                videocall_url: Some(String::from("https://videocallurl.com")),
                notes: Some(String::from("")),
                session_duration: Some(30),
                completed: false,
                created_at: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn create_with_id_fails() {
        let use_cases = SessionUseCases::new(Arc::new(MockSessionPersistence));

        let result = use_cases
            .create(&Session {
                id: Some(Uuid::new_v4()),
                patient_id: Uuid::new_v4(),
                professional_id: Uuid::new_v4(),
                session_type_id: Some(Uuid::new_v4()),
                session_status: SessionStatus::Scheduled,
                session_date: None,
                videocall_url: Some(String::from("https://videocallurl.com")),
                notes: Some(String::from("")),
                session_duration: Some(30),
                completed: false,
                created_at: None,
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn read_all_works() {
        let use_cases = SessionUseCases::new(Arc::new(MockSessionPersistence));

        let result = use_cases.read_all().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_patient_works() {
        let use_cases = SessionUseCases::new(Arc::new(MockSessionPersistence));

        let result = use_cases.read_patient(&Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_professional_works() {
        let use_cases = SessionUseCases::new(Arc::new(MockSessionPersistence));

        let result = use_cases.read_professional(&Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_single_works() {
        let use_cases = SessionUseCases::new(Arc::new(MockSessionPersistence));

        let result = use_cases.read_single(&Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_works() {
        let use_cases = SessionUseCases::new(Arc::new(MockSessionPersistence));

        let result = use_cases
            .update(&Session {
                id: Some(Uuid::new_v4()),
                patient_id: Uuid::new_v4(),
                professional_id: Uuid::new_v4(),
                session_type_id: Some(Uuid::new_v4()),
                session_status: SessionStatus::Scheduled,
                session_date: None,
                videocall_url: Some(String::from("https://videocallurl.com")),
                notes: Some(String::from("")),
                session_duration: Some(30),
                completed: false,
                created_at: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_works() {
        let use_cases = SessionUseCases::new(Arc::new(MockSessionPersistence));

        let result = use_cases.delete(&Uuid::new_v4()).await;

        assert!(result.is_ok());
    }
}
