use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};

use crate::{app_error::AppResult, entities::email::EmailKind};

#[async_trait]
pub trait EmailPersistence: Send + Sync {
    async fn add_email(
        &self,
        from: String,
        to: String,
        subject: String,
        body: String,
        kind: EmailKind,
    ) -> AppResult<()>;
}

#[derive(Clone)]
pub struct EmailUseCases {
    persistence: Arc<dyn EmailPersistence>,
}

impl EmailUseCases {
    pub fn new(persistence: Arc<dyn EmailPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn add_email(
        &self,
        from: String,
        to: String,
        subject: String,
        body: String,
        kind: EmailKind,
    ) -> AppResult<()> {
        info!("Attempting add email...");

        self.persistence
            .add_email(from, to, subject, body, kind)
            .await?;

        info!("Email added.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    #[allow(dead_code)]
    struct MockEmailPersistence;

    #[async_trait]
    impl EmailPersistence for MockEmailPersistence {
        async fn add_email(
            &self,
            from: String,
            to: String,
            subject: String,
            body: String,
            kind: EmailKind,
        ) -> AppResult<()> {
            assert_eq!(from, "testuser@gmail.com");
            assert_eq!(to, "testuser@gmail.com");
            assert_eq!(subject, "email subject");
            assert_eq!(body, "email body");
            assert_eq!(kind, EmailKind::Verification);

            Ok(())
        }
    }
}
