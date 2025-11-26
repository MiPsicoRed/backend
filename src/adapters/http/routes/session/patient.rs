use std::sync::Arc;

use axum::{Extension, Json, extract::{Query, State}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{AuthUser, Validateable, session::SessionResponse}, app_error::{AppError, AppResult}, entities::{patient::Patient, user::Role}, use_cases::{patient::PatientUseCases, session::SessionUseCases}
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
    description = "\n\n**Required:**  Verified Email + Admin/Professional Role or requesting patient_id"
)]
#[instrument(skip(patient_use_cases, session_use_cases))]
pub async fn read_patient_sessions(
    Extension(auth_user): Extension<AuthUser>,
    State(patient_use_cases): State<Arc<PatientUseCases>>,
    State(session_use_cases): State<Arc<SessionUseCases>>,
    Query(params): Query<SessionReadPatientQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read patient sessions called");
    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let patient_uuid = Uuid::parse_str(&params.patient_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

     let patient = patient_use_cases
        .read_single(&patient_uuid)
        .await?;

    let is_authorized = authorized(&auth_user, &patient);
    if !is_authorized {
        return Err(AppError::Unauthorized(
            String::from("You don't have permission for this endpoint")
        ));
    }

    let sessions = session_use_cases
        .read_patient(&patient_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(SessionReadPatientResponse { success:true , data: sessions.into_iter().map(Into::into).collect() }),
    ))
}

fn authorized(auth_user: &AuthUser, patient: &Patient) -> bool {
    let requesting_role = Role::from_id(auth_user.role_id).unwrap_or_default();
    
    // Check authorization
    match requesting_role {
        Role::Admin | Role::Professional => true,
        Role::Patient => {
            patient.user_id
                .as_ref()
                .map(|id| id.to_string() == auth_user.user_id)
                .unwrap_or(false) // Don't allow if no user_id specified
        }
    }
}