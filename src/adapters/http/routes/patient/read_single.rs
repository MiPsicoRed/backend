use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::{patient::PatientResponse, Validateable}, app_error::{AppError, AppResult}, use_cases::patient::PatientUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct ReadSinglePayload {
    user_id: String,
}

impl Validateable for ReadSinglePayload {
    fn valid(&self) -> bool {
        !self.user_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReadSingleResponse {
    data: PatientResponse,
    success: bool,
}

#[utoipa::path(get, path = "/api/patient/single", 
    responses( 
        (status = 200, description = "Data retrieved correctly", body = ReadSingleResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Patient",
    summary = "Deletes a patient"
)]
#[instrument(skip(use_cases))]
pub async fn read_single_patient(
    State(use_cases): State<Arc<PatientUseCases>>,
    Json(payload): Json<ReadSinglePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Read single patient called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let user_uuid = Uuid::parse_str(&payload.user_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let patient = use_cases
        .read_single(user_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(ReadSingleResponse { success:true , data: patient.into()}),
    ))
}
