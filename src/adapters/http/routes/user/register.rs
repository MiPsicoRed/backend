use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::{
    adapters::http::routes::Validateable,
    app_error::{AppError, AppResult},
    use_cases::user::UserUseCases,
};

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterPayload {
    username: String,
    email: String,
    password: SecretString,
}

impl Validateable for RegisterPayload {
    fn valid(&self) -> bool {
        !self.email.is_empty()
            && !self.password.expose_secret().is_empty()
            && !self.username.is_empty()
    }
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    success: bool,
}

/// Creates a new user based on the submitted credentials.
#[instrument(skip(user_use_cases))]
pub async fn register(
    State(user_use_cases): State<Arc<UserUseCases>>,
    Json(payload): Json<RegisterPayload>,
) -> AppResult<impl IntoResponse> {
    info!("Register user called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    user_use_cases
        .add(&payload.username, &payload.email, &payload.password)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse { success: true }),
    ))
}
