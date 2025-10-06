use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::{
    adapters::{http::routes::Validateable, persistence::user_token::UserTokenDb},
    app_error::{AppError, AppResult},
    use_cases::user_token::UserTokenUseCases,
};

#[derive(Debug, Clone, Deserialize)]
pub struct GeneratePayload {
    user_id: String,
}

impl Validateable for GeneratePayload {
    fn valid(&self) -> bool {
        !self.user_id.is_empty()
    }
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    success: bool,
    data: UserTokenDb,
}

/// Creates a new user based on the submitted credentials.
#[instrument(skip(user_token_use_cases))]
pub async fn generate_token(
    State(user_token_use_cases): State<Arc<UserTokenUseCases>>,
    Json(payload): Json<GeneratePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Generate user token called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let token = user_token_use_cases
        .generate_token_and_send_mail(&payload.user_id)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(GenerateResponse {
            success: true,
            data: token,
        }),
    ))
}
