use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

use crate::{
    adapters::http::routes::Validateable,
    app_error::{AppError, AppResult},
    use_cases::user::UserUseCases,
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct LoginPayload {
    email: String,
    #[schema(value_type = String, format = "password")]
    password: SecretString,
}

impl Validateable for LoginPayload {
    // TODO: Server validate email is valid email
    fn valid(&self) -> bool {
        !self.email.is_empty() && !self.password.expose_secret().is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    jwt: String,
    success: bool,
}

#[utoipa::path(post, path = "/api/user/login", 
    responses( 
        (status = 200, description = "Ok", body = LoginResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ), 
    tag = "User",
    summary = "Login as a specific user"
)]
#[instrument(skip(user_use_cases))]
pub async fn login(
    State(user_use_cases): State<Arc<UserUseCases>>,
    Json(payload): Json<LoginPayload>,
) -> AppResult<impl IntoResponse> {
    info!("Login user called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let jwt = user_use_cases
        .login(&payload.email, &payload.password)
        .await?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse { success: true, jwt }),
    ))
}
