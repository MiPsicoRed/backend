use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{ professional_language::ProfessionalLanguageResponse, Validateable}, app_error::{AppError, AppResult}, use_cases::professional_language::ProfessionalLanguageUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct ProfessionalLanguageReadSingleQuery {
    #[param(example = "insert-professional-language-uuid")]
    professional_language_id: String,
}

impl Validateable for ProfessionalLanguageReadSingleQuery {
    fn valid(&self) -> bool {
        !self.professional_language_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalLanguageReadSingleResponse {
    data: ProfessionalLanguageResponse,
    success: bool,
}

#[utoipa::path(get, path = "/api/professional_language/single", 
    params(ProfessionalLanguageReadSingleQuery),
    responses( 
        (status = 200, description = "Data retrieved correctly", body = ProfessionalLanguageReadSingleResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional Language",
    summary = "Retrieves data of a single professional language",
    description = "\n\n**Required:**  Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn read_single_professional_language(
    State(use_cases): State<Arc<ProfessionalLanguageUseCases>>,
    Query(params): Query<ProfessionalLanguageReadSingleQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read single professional language called");
    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let uuid = Uuid::parse_str(&params.professional_language_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let language = use_cases
        .read_single(uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProfessionalLanguageReadSingleResponse { success:true , data: language.into()}),
    ))
}
