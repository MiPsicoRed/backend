use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

use crate::{
    adapters::http::routes::professional_specialization::ProfessionalSpecializationResponse, app_error::AppResult, use_cases::professional_specialization::ProfessionalSpecializationUseCases
};

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalSpecializationReadAllResponse {
    data: Vec<ProfessionalSpecializationResponse>,
    success: bool,
}

#[utoipa::path(get, path = "/api/professional_specialization/all", 
    responses( 
        (status = 200, description = "Data retrieved correctly", body = ProfessionalSpecializationReadAllResponse),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional Specialization",
    summary = "Returns all professional specializations with their info",
    description = "\n\n**Required:** Verified Email + Admin Role"
)]
#[instrument(skip(use_cases))]
pub async fn read_all_professional_specializations(
    State(use_cases): State<Arc<ProfessionalSpecializationUseCases>>,
) -> AppResult<impl IntoResponse> {
    info!("Read all professional specializations called");

    let specializations = use_cases
        .read_all()
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProfessionalSpecializationReadAllResponse { success:true, data: specializations.into_iter().map(Into::into).collect() }),
    ))
}
