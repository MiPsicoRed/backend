use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, entities::professional_specialization::ProfessionalSpecialization, use_cases::professional_specialization::ProfessionalSpecializationUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ProfessionalSpecializationUpdatePayload {
    id: String,
    name: String,
}

impl Validateable for ProfessionalSpecializationUpdatePayload {
    fn valid(&self) -> bool {
        !self.name.is_empty() && !self.id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalSpecializationUpdateResponse {
    success: bool,
}

#[utoipa::path(patch, path = "/api/professional_specialization/update", 
    responses( 
        (status = 200, description = "Updated", body = ProfessionalSpecializationUpdateResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional Specialization",
    summary = "Updates a professional specialization",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn update_professional_specialization(
    State(use_cases): State<Arc<ProfessionalSpecializationUseCases>>,
    Json(payload): Json<ProfessionalSpecializationUpdatePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Update professional specialization called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let id = Uuid::parse_str(&payload.id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let specialization = ProfessionalSpecialization { id: Some(id), professional_id: None, name: payload.name,  created_at: None };

    use_cases
        .update(&specialization)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProfessionalSpecializationUpdateResponse { success:true }),
    ))
}
