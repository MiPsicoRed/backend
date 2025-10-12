use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, entities::professional_language::ProfessionalLanguage, use_cases::professional_language::ProfessionalLanguageUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ProfessionalLanguageUpdatePayload {
    id: String,
    name: String,
}

impl Validateable for ProfessionalLanguageUpdatePayload {
    fn valid(&self) -> bool {
        !self.name.is_empty() && !self.id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalLanguageUpdateResponse {
    success: bool,
}

#[utoipa::path(patch, path = "/api/professional_language/update", 
    responses( 
        (status = 200, description = "Updated", body = ProfessionalLanguageUpdateResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional Language",
    summary = "Updates a professional language",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn update_professional_language(
    State(use_cases): State<Arc<ProfessionalLanguageUseCases>>,
    Json(payload): Json<ProfessionalLanguageUpdatePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Update professional language called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let id = Uuid::parse_str(&payload.id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let language = ProfessionalLanguage { id: Some(id), professional_id: None, name: payload.name,  created_at: None };

    use_cases
        .update(&language)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProfessionalLanguageUpdateResponse { success:true }),
    ))
}
