use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{session::SessionResponse, Validateable}, app_error::{AppError, AppResult}, use_cases::session::SessionUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct SessionReadPatientQuery {
    #[param(example = "insert-patient-uuid")]
    patient_id: String,
}

impl Validateable for SessionReadPatientQuery {
    fn valid(&self) -> bool {
        !self.patient_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionReadPatientResponse {
    data: Vec<SessionResponse>,
    success: bool,
}

#[utoipa::path(get, path = "/api/session/patient", 
    params(SessionReadPatientQuery),
    responses( 
        (status = 200, description = "Data retrieved correctly", body = SessionReadPatientResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Session",
    summary = "Retrieves data of all sessions of a given patient",
    description = "\n\n**Required:**  Verified Email + Admin/Patient Role"
)]
#[instrument(skip(use_cases))]
pub async fn read_patient_sessions(
    State(use_cases): State<Arc<SessionUseCases>>,
    Query(params): Query<SessionReadPatientQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read patient sessions called");
    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let patient_uuid = Uuid::parse_str(&params.patient_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let sessions = use_cases
        .read_patient(&patient_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(SessionReadPatientResponse { success:true , data: sessions.into_iter().map(Into::into).collect() }),
    ))
}