use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    app_error::{AppError, AppResult},
    entities::session::Session,
    infra::config::AppConfig,
    use_cases::session::VideocallProvider,
};

#[derive(Deserialize, Clone)]
pub struct GoogleServiceAccount {
    pub client_email: String,
    pub private_key: String,
}

#[derive(Serialize)]
struct GoogleJwtClaims {
    iss: String,
    scope: String,
    aud: String,
    exp: u64,
    iat: u64,
}

#[derive(Deserialize)]
struct GoogleAuthResponse {
    access_token: String,
}

pub struct GoogleMeetService {
    config: Arc<AppConfig>,
    client: Client,
}

impl GoogleMeetService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    async fn get_access_token(&self, credentials_path: &str) -> AppResult<String> {
        let creds_content = tokio::fs::read_to_string(credentials_path)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to read google credentials: {}", e)))?;
        
        let creds: GoogleServiceAccount = serde_json::from_str(&creds_content)
            .map_err(|e| AppError::Internal(format!("Invalid google credentials format: {}", e)))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = GoogleJwtClaims {
            iss: creds.client_email,
            scope: "https://www.googleapis.com/auth/calendar.events".to_string(),
            aud: "https://oauth2.googleapis.com/token".to_string(),
            exp: now + 3600,
            iat: now,
        };

        let mut header = Header::new(Algorithm::RS256);
        header.typ = Some("JWT".to_string());

        let encoding_key = EncodingKey::from_rsa_pem(creds.private_key.as_bytes())
            .map_err(|e| AppError::Internal(format!("Failed to parse RSA key: {}", e)))?;

        let jwt = encode(&header, &claims, &encoding_key)
            .map_err(|e| AppError::Internal(format!("Failed to sign JWT: {}", e)))?;

        let params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ];

        let res = self.client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to request token: {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            error!("Google Auth failed ({}): {}", status, text);
            return Err(AppError::Internal("Google Auth failed".into()));
        }

        let auth_res: GoogleAuthResponse = res.json().await
            .map_err(|e| AppError::Internal(format!("Failed to parse Google Auth response: {}", e)))?;

        Ok(auth_res.access_token)
    }
}

#[async_trait]
impl VideocallProvider for GoogleMeetService {
    async fn generate_videocall_url(&self, session: &Session) -> AppResult<String> {
        let creds_path = match &self.config.google_application_credentials {
            Some(path) if !path.is_empty() => path,
            _ => {
                info!("Google Meet disabled: NO GOOGLE_APPLICATION_CREDENTIALS set");
                return Ok("".to_string()); // Or raise an error
            }
        };

        let access_token = self.get_access_token(creds_path).await?;
        let request_id = Uuid::new_v4().to_string();

        let start_time = session.session_date.unwrap_or_else(|| chrono::Utc::now().naive_utc());
        let end_time = start_time + chrono::Duration::minutes(session.session_duration.unwrap_or(60) as i64);

        let start_time_str = start_time.and_utc().to_rfc3339();
        let end_time_str = end_time.and_utc().to_rfc3339();

        let payload = serde_json::json!({
            "summary": "Mipsicored Session",
            "description": "Therapy Session Videocall",
            "start": { "dateTime": start_time_str },
            "end": { "dateTime": end_time_str },
            "conferenceData": {
                "createRequest": {
                    "requestId": request_id,
                    "conferenceSolutionKey": {
                        "type": "hangoutsMeet"
                    }
                }
            }
        });

        let res = self.client
            .post("https://www.googleapis.com/calendar/v3/calendars/primary/events?conferenceDataVersion=1")
            .bearer_auth(access_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to create meet event: {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            error!("Google Calendar failed ({}): {}", status, text);
            return Err(AppError::Internal("Google Calendar Meet Creation Failed".into()));
        }

        #[derive(Deserialize)]
        struct ConfData {
            #[serde(rename = "hangoutLink")]
            hangout_link: Option<String>,
        }

        #[derive(Deserialize)]
        struct CreateEventResponse {
            #[serde(rename = "hangoutLink")]
            hangout_link_fallback: Option<String>,
            #[serde(default)]
            #[serde(rename = "conferenceData")]
            conference_data: Option<ConfData>,
        }

        let event_res: CreateEventResponse = res.json().await
            .map_err(|e| AppError::Internal(format!("Failed to parse meet response: {}", e)))?;

        let meet_link = event_res.hangout_link_fallback
            .or_else(|| event_res.conference_data.and_then(|c| c.hangout_link))
            .unwrap_or_else(|| "".to_string());

        Ok(meet_link)
    }
}
