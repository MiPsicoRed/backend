
use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, use_cases::professional::ProfessionalUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ProfessionalDeletePayload {
    professional_id: String,
}

impl Validateable for ProfessionalDeletePayload {
    fn valid(&self) -> bool {
        !self.professional_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalDeleteResponse {
    success: bool,
}

#[utoipa::path(delete, path = "/api/professional/delete", 
    responses( 
        (status = 200, description = "Deleted", body = ProfessionalDeleteResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional",
    summary = "Deletes a professional",
    description = "\n\n**Required:** Verified Email + Admin Role"
)]
#[instrument(skip(use_cases))]
pub async fn delete_professional(
    State(use_cases): State<Arc<ProfessionalUseCases>>,
    Json(payload): Json<ProfessionalDeletePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Delete professional called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let professional_uuid = Uuid::parse_str(&payload.professional_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    use_cases
        .delete(professional_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProfessionalDeleteResponse { success:true }),
    ))
}
