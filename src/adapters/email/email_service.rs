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
    async fn send_verification_email(
        &self,
        to: &[String],
        token: &str,
    ) -> AppResult<(String, String)> {
        let body = verification_email_html(token);

        let email = CreateEmailBaseOptions::new(
            &self.config.resend_from_email,
            to,
            "Please Verify your Account",
        )
        .with_html(&body);

        self.client
            .emails
            .send(email)
            .await
            .map_err(|e| AppError::Internal(format!("Error sending mail: {}", e)))?;

        Ok((self.config.resend_from_email.clone(), body))
    }
}

fn verification_email_html(token: &str) -> String {
    // TODO: This should not be hardcoded here
    let verify_url = format!(
        "https://localhost:3001/api/user_token/verify?token={}",
        token
    );

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>Verify Your Account</title>
        </head>
        <body style="font-family: Arial, sans-serif; background-color: #f9f9f9; margin:0; padding:0;">
            <table width="100%" cellpadding="0" cellspacing="0" style="background-color:#f9f9f9; padding: 40px 0;">
                <tr>
                    <td align="center">
                        <table width="600" cellpadding="0" cellspacing="0" style="background:#ffffff; border-radius:8px; padding:40px; box-shadow:0 2px 6px rgba(0,0,0,0.1);">
                            <tr>
                                <td align="center" style="font-size:24px; font-weight:bold; color:#333333; padding-bottom:20px;">
                                    Verify Your Account
                                </td>
                            </tr>
                            <tr>
                                <td style="font-size:16px; color:#555555; text-align:center; padding-bottom:30px;">
                                    Thanks for signing up! Please confirm your email address by clicking the button below.
                                </td>
                            </tr>
                            <tr>
                                <td align="center" style="padding-bottom:30px;">
                                    <a href="{verify_url}" style="background-color:#4CAF50; color:#ffffff; text-decoration:none; padding:14px 28px; border-radius:6px; font-size:16px; display:inline-block;">
                                        Verify Email
                                    </a>
                                </td>
                            </tr>
                            <tr>
                                <td style="font-size:14px; color:#999999; text-align:center;">
                                    If the button doesn’t work, copy and paste this link into your browser:<br/>
                                    <a href="{verify_url}" style="color:#4CAF50; word-break:break-all;">{verify_url}</a>
                                </td>
                            </tr>
                        </table>
                    </td>
                </tr>
            </table>
        </body>
        </html>
        "#,
        verify_url = verify_url
    )
}
