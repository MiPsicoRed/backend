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
    pub user_id: String,
    pub user_type: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub birthdate: Option<String>,
    pub reason: Option<String>,
    pub experience: Option<String>,
    pub is_monoparental: Option<bool>,
    pub guardian_name: Option<String>,
    pub guardian_id_document: Option<String>,
    pub signature: Option<String>,
    pub guardian_2_name: Option<String>,
    pub guardian_2_id_document: Option<String>,
    pub signature_2: Option<String>,
}

impl Validateable for OnboardPayload {
    fn valid(&self) -> bool {
        !self.user_id.is_empty() && !self.user_type.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OnboardResponse {
    success: bool,
}

#[utoipa::path(post, path = "/api/user/onboarded", 
    responses( 
        (status = 201, description = "Onboarded", body = OnboardResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "User",
    summary = "Marks the user as onboarded on the database (the changes will not we reflected on the jwt until the user relogs) \n\n
        **Required:** Verified Email"
)]
#[instrument(skip(user_use_cases))]
pub async fn onboard_user(
    State(user_use_cases): State<Arc<UserUseCases>>,
    Json(payload): Json<OnboardPayload>,
) -> AppResult<impl IntoResponse> {
    info!("Onboard user called");
    // TODO: (NOTE): Right now any logged in user can mark anyone as onboarded with a post request if they have the desired user_id, 
    // they should not have access to other users_id's so this is probably okay.

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let user_uuid = Uuid::parse_str(&payload.user_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let dto = crate::use_cases::user::OnboardingDto {
        user_id: user_uuid,
        user_type: payload.user_type,
        full_name: payload.full_name,
        phone: payload.phone,
        birthdate: payload.birthdate.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
        reason: payload.reason,
        experience: payload.experience,
        is_monoparental: payload.is_monoparental.unwrap_or(true),
        guardian_name: payload.guardian_name,
        guardian_id_document: payload.guardian_id_document,
        signature: payload.signature,
        guardian2_name: payload.guardian_2_name,
        guardian2_id_document: payload.guardian_2_id_document,
        signature2: payload.signature_2,
    };

    user_use_cases
        .user_onboarded(dto)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(OnboardResponse { success:true }),
    ))
}
