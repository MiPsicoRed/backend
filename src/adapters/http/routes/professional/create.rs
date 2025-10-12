use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, entities::{gender::Gender, professional::Professional}, use_cases::professional::ProfessionalUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ProfessionalCreatePayload {
    user_id: String,
    gender_id: i32,
    birthdate: Option<chrono::NaiveDate>,
    license_number: Option<String>,
    bio: Option<String>,
    education: Option<String>,
    experience_years: Option<i32>,
    hourly_rate: Option<f32>,
    accepts_insurance: bool,
}


impl Validateable for ProfessionalCreatePayload {
    fn valid(&self) -> bool {
        self.birthdate.is_some() && !self.user_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalCreateResponse {
    success: bool,
}

#[utoipa::path(post, path = "/api/professional/create", 
    responses( 
        (status = 201, description = "Created", body = ProfessionalCreateResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ), 
    tag = "Professional",
    summary = "Creates a new professional",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn create_professional(
    State(use_cases): State<Arc<ProfessionalUseCases>>,
    Json(payload): Json<ProfessionalCreatePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Create professional called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    // Make sure the uuids are valid
    let user_uuid = Uuid::parse_str(&payload.user_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let professional = Professional { id: None, user_id: Some(user_uuid), gender: Gender::from_id(payload.gender_id).unwrap_or_default(), birthdate: payload.birthdate, license_number: payload.license_number, bio: payload.bio, education: payload.education, experience_years: payload.experience_years, hourly_rate: payload.hourly_rate, accepts_insurance: payload.accepts_insurance, created_at: None };

    use_cases
        .create(&professional)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(ProfessionalCreateResponse { success:true }),
    ))
}