
use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, use_cases::{professional_language::ProfessionalLanguageUseCases}
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ProfessionalLanguageDeletePayload {
    professional_language_id: String,
}

impl Validateable for ProfessionalLanguageDeletePayload {
    fn valid(&self) -> bool {
        !self.professional_language_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalLanguageDeleteResponse {
    success: bool,
}

#[utoipa::path(delete, path = "/api/professional_language/delete", 
    responses( 
        (status = 200, description = "Deleted", body = ProfessionalLanguageDeleteResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional Language",
    summary = "Deletes a professional language",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn delete_professional_language(
    State(use_cases): State<Arc<ProfessionalLanguageUseCases>>,
    Json(payload): Json<ProfessionalLanguageDeletePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Delete professional language called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let uuid = Uuid::parse_str(&payload.professional_language_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    use_cases
        .delete(uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProfessionalLanguageDeleteResponse { success:true }),
    ))
}
