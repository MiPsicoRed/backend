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
pub struct LoginPayload {
    username: String,
    password: SecretString,
}

impl Validateable for LoginPayload {
    fn valid(&self) -> bool {
        !self.username.is_empty() && !self.password.expose_secret().is_empty()
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    jwt: String,
    success: bool,
}

/// Attempts to login as the specified user
#[instrument(skip(user_use_cases))]
pub async fn login(
    State(user_use_cases): State<Arc<UserUseCases>>,
    Json(payload): Json<LoginPayload>,
) -> AppResult<impl IntoResponse> {
    info!("Register user called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let jwt = user_use_cases
        .login(&payload.username, &payload.password)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(LoginResponse { success: true, jwt }),
    ))
}
