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
    usersurname: String,
    email: String,
    phone: String,
    birthdate: Option<chrono::NaiveDate>,
    password: SecretString,
}

impl Validateable for RegisterPayload {
    // TODO: Server validate email is valid email
    fn valid(&self) -> bool {
        !self.email.is_empty()
            && !self.password.expose_secret().is_empty()
            && !self.username.is_empty()
            && !self.usersurname.is_empty()
            && !self.phone.is_empty()
            && self.birthdate.is_some()
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
        .add(
            &payload.username,
            &payload.usersurname,
            &payload.email,
            &payload.phone,
            payload.birthdate,
            &payload.password,
        )
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse { success: true }),
    ))
}
