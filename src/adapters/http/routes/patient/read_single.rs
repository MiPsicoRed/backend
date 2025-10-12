use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{patient::PatientResponse, AuthUser, Validateable}, app_error::{AppError, AppResult}, entities::{patient::Patient, user::Role}, use_cases::patient::PatientUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct PatientReadSingleQuery {
    #[param(example = "insert-patient-uuid")]
    patient_id: String,
}

impl Validateable for PatientReadSingleQuery {
    fn valid(&self) -> bool {
        !self.patient_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PatientReadSingleResponse {
    data: PatientResponse,
    success: bool,
}

#[utoipa::path(get, path = "/api/patient/single", 
    params(PatientReadSingleQuery),
    responses( 
        (status = 200, description = "Data retrieved correctly", body = PatientReadSingleResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Patient",
    summary = "Retrieves data of a single patient",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role or requesting user_id"
)]
#[instrument(skip(use_cases))]
pub async fn read_single_patient(
    Extension(auth_user): Extension<AuthUser>,
    State(use_cases): State<Arc<PatientUseCases>>,
    Query(params): Query<PatientReadSingleQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read single patient called");
    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let patient_uuid = Uuid::parse_str(&params.patient_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let patient = use_cases
        .read_single(patient_uuid)
        .await?;

    let is_authorized = authorized(&auth_user, &patient);
    if !is_authorized {
        return Err(AppError::Unauthorized(
            String::from("You don't have permission for this endpoint")
        ));
    }

    Ok((
        StatusCode::OK,
        Json(PatientReadSingleResponse { success:true , data: patient.into()}),
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