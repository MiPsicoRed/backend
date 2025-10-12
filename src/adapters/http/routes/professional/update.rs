use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, entities::{gender::Gender, professional::Professional}, use_cases::professional::ProfessionalUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ProfessionalUpdatePayload {
    id: String,
    gender_id: i32,
    birthdate: Option<chrono::NaiveDate>,
    license_number: Option<String>,
    bio: Option<String>,
    education: Option<String>,
    experience_years: Option<i32>,
    hourly_rate: Option<f32>,
    accepts_insurance: bool,
}

impl Validateable for ProfessionalUpdatePayload {
    fn valid(&self) -> bool {
        self.birthdate.is_some() && !self.id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalUpdateResponse {
    success: bool,
}

#[utoipa::path(patch, path = "/api/professional/update", 
    responses( 
        (status = 200, description = "Updated", body = ProfessionalUpdateResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional",
    summary = "Updates a professional",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn update_professional(
    State(use_cases): State<Arc<ProfessionalUseCases>>,
    Json(payload): Json<ProfessionalUpdatePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Update professional called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let id = Uuid::parse_str(&payload.id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let professional = Professional { id: Some(id), user_id: None, gender: Gender::from_id(payload.gender_id).unwrap_or_default(), birthdate: payload.birthdate, license_number: payload.license_number, bio: payload.bio, education: payload.education, experience_years: payload.experience_years, hourly_rate: payload.hourly_rate, accepts_insurance: payload.accepts_insurance, created_at: None };

    use_cases
        .update(&professional)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProfessionalUpdateResponse { success:true }),
    ))
}
