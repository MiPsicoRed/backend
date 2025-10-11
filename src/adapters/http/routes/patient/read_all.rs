use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

use crate::{
    adapters::http::routes::{patient::PatientResponse}, app_error::{AppResult}, use_cases::patient::PatientUseCases
};

#[derive(Debug, Serialize, ToSchema)]
pub struct PatientReadAllResponse {
    data: Vec<PatientResponse>,
    success: bool,
}

#[utoipa::path(get, path = "/api/patient/all", 
    responses( 
        (status = 200, description = "Data retrieved correctly", body = PatientReadAllResponse),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Patient",
    summary = "Returns all patients with their info",
    description = "\n\n**Required:** Verified Email + Admin Role"
)]
#[instrument(skip(use_cases))]
pub async fn read_all_patients(
    State(use_cases): State<Arc<PatientUseCases>>,
) -> AppResult<impl IntoResponse> {
    info!("Read all patients called");

    let patients = use_cases
        .read_all()
        .await?;

    Ok((
        StatusCode::OK,
        Json(PatientReadAllResponse { success:true, data: patients.into_iter().map(Into::into).collect() }),
    ))
}
