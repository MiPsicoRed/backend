use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    app_error::AppResult, use_cases::professional::ProfessionalUseCases, dtos::professional::selector::ProfessionalSelectorDTO
};

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalSelectorResponseData {
    professional_id: Uuid,
    name: String,
}

impl From<ProfessionalSelectorDTO> for ProfessionalSelectorResponseData {
    fn from(value: ProfessionalSelectorDTO) -> Self {
        Self { professional_id: value.professional_id, name: value.name }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalSelectorResponse {
    data: Vec<ProfessionalSelectorResponseData>,
    success: bool,
}

#[utoipa::path(get, path = "/api/professional/selector", 
    responses( 
        (status = 200, description = "Data retrieved correctly", body = ProfessionalSelectorResponse),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional",
    summary = "Returns all professionals names",
    description = "\n\n**Required:** Verified Email"
)]
#[instrument(skip(use_cases))]
pub async fn professionals_selector(
    State(use_cases): State<Arc<ProfessionalUseCases>>,
) -> AppResult<impl IntoResponse> {
    info!("Professionals selector called");

    let professionals = use_cases
        .selector()
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProfessionalSelectorResponse { success:true, data: professionals.into_iter().map(Into::into).collect() }),
    ))
}
