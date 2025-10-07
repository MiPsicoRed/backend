use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};

use crate::{
    adapters::http::routes::Validateable,
    app_error::{AppError, AppResult},
    use_cases::user_token::UserTokenUseCases,
};

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct VerifyQuery {
    #[param(example = "f99d8d694c4aff731ba577a4c0e647d498794c7f...")]
    token: String,
}

impl Validateable for VerifyQuery {
    fn valid(&self) -> bool {
        !self.token.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyResponse {
    success: bool,
}

#[utoipa::path(post, path = "/api/user_token/verify", 
    params(VerifyQuery),
    responses( 
        (status = 202, description = "Accepted", body = VerifyResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ), 
    tag = "User Token",
    summary = "Verifies the given token"
)]
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
