use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, use_cases::patient::PatientUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct DeletePayload {
    patient_id: String,
}

impl Validateable for DeletePayload {
    fn valid(&self) -> bool {
        !self.patient_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteResponse {
    success: bool,
}

#[utoipa::path(delete, path = "/api/patient/delete", 
    responses( 
        (status = 200, description = "Deleted", body = DeleteResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Patient",
    summary = "Deletes a patient",
    description = "\n\n**Required:** Verified Email + Admin Role"
)]
#[instrument(skip(use_cases))]
pub async fn delete_patient(
    State(use_cases): State<Arc<PatientUseCases>>,
    Json(payload): Json<DeletePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Delete patient called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let patient_uuid = Uuid::parse_str(&payload.patient_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    use_cases
        .delete(patient_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(DeleteResponse { success:true }),
    ))
}
