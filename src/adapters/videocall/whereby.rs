use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::{
    app_error::{AppError, AppResult},
    infra::config::AppConfig,
    use_cases::session::VideoCallService,
};

pub struct WherebyService {
    config: Arc<AppConfig>,
    client: reqwest::Client,
}

impl WherebyService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateMeetingRequest {
    end_date: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateMeetingResponse {
    room_url: String,
}

#[async_trait]
impl VideoCallService for WherebyService {
    #[instrument(skip(self))]
    async fn create_meeting(&self, end_date: chrono::NaiveDateTime) -> AppResult<String> {
        info!("Sending request to Whereby API...");

        let url = "https://api.whereby.dev/v1/meetings";
        let api_key = &self.config.whereby_key;

        // Whereby expects ISO 8601 with timezone, e.g. 2026-05-30T15:00:00Z
        let end_date_str = end_date.format("%Y-%m-%dT%H:%M:%SZ").to_string();

        let request_body = CreateMeetingRequest {
            end_date: end_date_str,
        };

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to send request to Whereby: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::Internal(format!(
                "Whereby API error ({}): {}",
                status, error_text
            )));
        }

        let result: CreateMeetingResponse = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse Whereby response: {}", e)))?;

        info!("Successfully created Whereby meeting: {}", result.room_url);

        Ok(result.room_url)
    }
}
