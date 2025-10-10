use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, entities::{gender::Gender, sexual_orientation::SexualOrientation}, use_cases::patient::PatientUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdatePayload {
    id: String,
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

impl Validateable for UpdatePayload {
    fn valid(&self) -> bool {
        self.birthdate.is_some() && !self.phone.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UpdateResponse {
    success: bool,
}

#[utoipa::path(patch, path = "/api/patient/update", 
    responses( 
        (status = 200, description = "Updated", body = UpdateResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Patient",
    summary = "Updates a patient",
    description = "\n\n**Required:** Verified Email"
)]
#[instrument(skip(use_cases))]
pub async fn update_patient(
    State(use_cases): State<Arc<PatientUseCases>>,
    Json(payload): Json<UpdatePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Update patient called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let id = Uuid::parse_str(&payload.id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    use_cases
        .update(id, Gender::from_id(payload.gender_id).unwrap_or_default(), SexualOrientation::from_id(payload.sexual_orientation_id).unwrap_or_default(), payload.birthdate, payload.phone, payload.emergency_contact_name, payload.emergency_contact_phone, payload.insurance_policy_number, payload.medical_history, payload.current_medications, payload.allergies)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(UpdateResponse {success:true }),
    ))
}
