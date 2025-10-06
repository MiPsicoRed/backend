use std::sync::Arc;

use async_trait::async_trait;
use resend_rs::{Resend, types::CreateEmailBaseOptions};

use crate::{
    app_error::{AppError, AppResult},
    infra::config::AppConfig,
    use_cases::user_token::UserTokenEmailService,
};

pub struct EmailService {
    config: Arc<AppConfig>,
    client: Arc<Resend>,
}

impl EmailService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        let client = Arc::new(Resend::new(&config.resend_key));

        Self { config, client }
    }
}

#[async_trait]
impl UserTokenEmailService for EmailService {
    async fn send_email(
        &self,
        from: &str,
        to: &[String],
        subject: &str,
        email_html: &str,
    ) -> AppResult<()> {
        let email = CreateEmailBaseOptions::new(from, to, subject).with_html(email_html);

        self.client
            .emails
            .send(email)
            .await
            .map_err(|e| AppError::Internal(format!("Error sending mail: {}", e)))?;

        Ok(())
    }
}
