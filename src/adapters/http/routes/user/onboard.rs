use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable,
    app_error::{AppError, AppResult},
    use_cases::user::UserUseCases,
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct OnboardPayload {
    user_id: String,
}

impl Validateable for OnboardPayload {
    fn valid(&self) -> bool {
        !self.user_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OnboardResponse {
    success: bool,
}

#[utoipa::path(post, path = "/api/user/onboard", 
    responses( 
        (status = 201, description = "Onboarded", body = OnboardResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ), 
    tag = "User",
    summary = "Marks the user as onboarded on the database (the changes will not we reflected on the jwt until the user relogs)"
)]
#[instrument(skip(user_use_cases))]
pub async fn onboard_user(
    State(user_use_cases): State<Arc<UserUseCases>>,
    Json(payload): Json<OnboardPayload>,
) -> AppResult<impl IntoResponse> {
    info!("Onboard user called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let user_uuid = Uuid::parse_str(&payload.user_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    user_use_cases
        .user_onboarded(&user_uuid)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(OnboardResponse { success:true }),
    ))
}
