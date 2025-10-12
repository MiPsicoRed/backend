use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

use crate::{
    adapters::http::routes::professional_language::ProfessionalLanguageResponse, app_error::AppResult, use_cases::professional_language::ProfessionalLanguageUseCases
};

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalLanguageReadAllResponse {
    data: Vec<ProfessionalLanguageResponse>,
    success: bool,
}

#[utoipa::path(get, path = "/api/professional_language/all", 
    responses( 
        (status = 200, description = "Data retrieved correctly", body = ProfessionalLanguageReadAllResponse),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional Language",
    summary = "Returns all professional languages with their info",
    description = "\n\n**Required:** Verified Email + Admin Role"
)]
#[instrument(skip(use_cases))]
pub async fn read_all_professional_languages(
    State(use_cases): State<Arc<ProfessionalLanguageUseCases>>,
) -> AppResult<impl IntoResponse> {
    info!("Read all professional languages called");

    let languages = use_cases
        .read_all()
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProfessionalLanguageReadAllResponse { success:true, data: languages.into_iter().map(Into::into).collect() }),
    ))
}
