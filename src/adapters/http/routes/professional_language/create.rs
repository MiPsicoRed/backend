use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, entities::professional_language::ProfessionalLanguage, use_cases::professional_language::ProfessionalLanguageUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ProfessionalLanguageCreatePayload {
    professional_id: String,
    name: String,
}


impl Validateable for ProfessionalLanguageCreatePayload {
    fn valid(&self) -> bool {
        !self.professional_id.is_empty() && !self.name.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalLanguageCreateResponse {
    success: bool,
}

#[utoipa::path(post, path = "/api/professional_language/create", 
    responses( 
        (status = 201, description = "Created", body = ProfessionalLanguageCreateResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ), 
    tag = "Professional Language",
    summary = "Creates a new professional language",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn create_professional_language(
    State(use_cases): State<Arc<ProfessionalLanguageUseCases>>,
    Json(payload): Json<ProfessionalLanguageCreatePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Create professional language called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    // Make sure the uuids are valid
    let professional_uuid = Uuid::parse_str(&payload.professional_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let language = ProfessionalLanguage { id: None, professional_id: Some(professional_uuid), name: payload.name, created_at: None };

    use_cases
        .create(&language)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(ProfessionalLanguageCreateResponse { success:true }),
    ))
}