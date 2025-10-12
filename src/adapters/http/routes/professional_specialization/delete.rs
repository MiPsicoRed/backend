
use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, use_cases::{professional_specialization::ProfessionalSpecializationUseCases}
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ProfessionalSpecializationDeletePayload {
    professional_specialization_id: String,
}

impl Validateable for ProfessionalSpecializationDeletePayload {
    fn valid(&self) -> bool {
        !self.professional_specialization_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalSpecializationDeleteResponse {
    success: bool,
}

#[utoipa::path(delete, path = "/api/professional_specialization/delete", 
    responses( 
        (status = 200, description = "Deleted", body = ProfessionalSpecializationDeleteResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional Specialization",
    summary = "Deletes a professional specialization",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn delete_professional_specialization(
    State(use_cases): State<Arc<ProfessionalSpecializationUseCases>>,
    Json(payload): Json<ProfessionalSpecializationDeletePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Delete professional specialization called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let uuid = Uuid::parse_str(&payload.professional_specialization_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    use_cases
        .delete(uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProfessionalSpecializationDeleteResponse { success:true }),
    ))
}
