use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::{
    adapters::http::routes::Validateable,
    app_error::{AppError, AppResult},
    use_cases::user_token::UserTokenUseCases,
};

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyQuery {
    token: String,
}

impl Validateable for VerifyQuery {
    fn valid(&self) -> bool {
        !self.token.is_empty()
    }
}

#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    success: bool,
}

/// Creates a new user based on the submitted credentials.
#[instrument(skip(user_token_use_cases))]
pub async fn verify(
    State(user_token_use_cases): State<Arc<UserTokenUseCases>>,
    Query(params): Query<VerifyQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Verify user token called");

    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    user_token_use_cases.verify_token(&params.token).await?;

    Ok((StatusCode::ACCEPTED, Json(VerifyResponse { success: true })))
}
