use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{ professional_specialization::ProfessionalSpecializationResponse, Validateable}, app_error::{AppError, AppResult}, use_cases::professional_specialization::ProfessionalSpecializationUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct ProfessionalSpecializationReadSingleQuery {
    #[param(example = "insert-professional-specialization-uuid")]
    professional_specialization_id: String,
}

impl Validateable for ProfessionalSpecializationReadSingleQuery {
    fn valid(&self) -> bool {
        !self.professional_specialization_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalSpecializationReadSingleResponse {
    data: ProfessionalSpecializationResponse,
    success: bool,
}

#[utoipa::path(get, path = "/api/professional_specialization/single", 
    params(ProfessionalSpecializationReadSingleQuery),
    responses( 
        (status = 200, description = "Data retrieved correctly", body = ProfessionalSpecializationReadSingleResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional Specialization",
    summary = "Retrieves data of a single professional specialization",
    description = "\n\n**Required:**  Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn read_single_professional_specialization(
    State(use_cases): State<Arc<ProfessionalSpecializationUseCases>>,
    Query(params): Query<ProfessionalSpecializationReadSingleQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read single professional specialization called");
    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let uuid = Uuid::parse_str(&params.professional_specialization_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let specialization = use_cases
        .read_single(uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProfessionalSpecializationReadSingleResponse { success:true , data: specialization.into()}),
    ))
}
