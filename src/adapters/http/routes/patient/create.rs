use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::{AuthUser, Validateable}, app_error::{AppError, AppResult}, entities::{gender::Gender, sexual_orientation::SexualOrientation, user::Role}, use_cases::patient::PatientUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PatientCreatePayload {
    user_id: Option<String>,
    gender_id: i32,
    sexual_orientation_id: i32,
    birthdate: Option<chrono::NaiveDate>,
    phone: String,
    emergency_contact_name: Option<String>,
    emergency_contact_phone: Option<String>,
    insurance_policy_number: Option<String>,
    medical_history: Option<String>,
    current_medications: Option<String>,
    allergies: Option<String>,
}

impl Validateable for PatientCreatePayload {
    fn valid(&self) -> bool {
        self.birthdate.is_some() && !self.phone.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PatientCreateResponse {
    success: bool,
}

#[utoipa::path(post, path = "/api/patient/create", 
    responses( 
        (status = 201, description = "Created", body = PatientCreateResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ), 
    tag = "Patient",
    summary = "Creates a new patient",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role OR Creating for requesting user_id"
)]
#[instrument(skip(use_cases))]
pub async fn create_patient(
    Extension(auth_user): Extension<AuthUser>,
    State(use_cases): State<Arc<PatientUseCases>>,
    Json(payload): Json<PatientCreatePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Create patient called");
    let is_authorized = authorized(&auth_user, &payload);
    if !is_authorized {
        return Err(AppError::Unauthorized(
            String::from("You don't have permission for this endpoint")
        ));
    }

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    // If the user is informed we make sure is valid uuid first
    let user_uuid = payload.user_id
    .map(|uid| Uuid::parse_str(&uid).map_err(|_| AppError::Internal("Invalid UUID string".into())))
    .transpose()?;

    use_cases
        .create(user_uuid, Gender::from_id(payload.gender_id).unwrap_or_default(), SexualOrientation::from_id(payload.sexual_orientation_id).unwrap_or_default(), payload.birthdate, &payload.phone, payload.emergency_contact_name, payload.emergency_contact_phone, payload.insurance_policy_number, payload.medical_history, payload.current_medications, payload.allergies)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(PatientCreateResponse { success:true }),
    ))
}

fn authorized(auth_user: &AuthUser, payload: &PatientCreatePayload) -> bool {
    let requesting_role = Role::from_id(auth_user.role_id).unwrap_or_default();
    
    // Check authorization
    match requesting_role {
        Role::Admin | Role::Professional => true,
        Role::Patient => {
            payload.user_id
                .as_ref()
                .map(|id| id == &auth_user.user_id)
                .unwrap_or(false) // Don't allow if no user_id specified
        }
    }
}