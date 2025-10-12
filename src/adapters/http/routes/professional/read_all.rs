use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

use crate::{
    adapters::http::routes::professional::ProfessionalResponse, app_error::AppResult, use_cases::professional::ProfessionalUseCases
};

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalReadAllResponse {
    data: Vec<ProfessionalResponse>,
    success: bool,
}

#[utoipa::path(get, path = "/api/professional/all", 
    responses( 
        (status = 200, description = "Data retrieved correctly", body = ProfessionalReadAllResponse),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional",
    summary = "Returns all professionals with their info",
    description = "\n\n**Required:** Verified Email + Admin Role"
)]
#[instrument(skip(use_cases))]
pub async fn read_all_professionals(
    State(use_cases): State<Arc<ProfessionalUseCases>>,
) -> AppResult<impl IntoResponse> {
    info!("Read all professionals called");

    let professionals = use_cases
        .read_all()
        .await?;

    Ok((
        StatusCode::OK,
        Json(ProfessionalReadAllResponse { success:true, data: professionals.into_iter().map(Into::into).collect() }),
    ))
}
