use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, entities:: session::{Session, SessionStatus}, use_cases::session::SessionUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct SessionCreatePayload {
    patient_id: String,
    professional_id: String,
    session_type_id: Option<String>,
    session_status_id: Option<i32>,
    session_date: Option<chrono::NaiveDateTime>,
    videocall_url: Option<String>,
    notes: Option<String>,
    session_duration: Option<i32>,
}

impl Validateable for SessionCreatePayload {
    fn valid(&self) -> bool {
        !self.patient_id.is_empty() && !self.professional_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionCreateResponse {
    success: bool,
}

#[utoipa::path(post, path = "/api/session/create", 
    responses( 
        (status = 201, description = "Created", body = SessionCreateResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ), 
    tag = "Session",
    summary = "Creates a new session",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn create_session(
    State(use_cases): State<Arc<SessionUseCases>>,
    Json(payload): Json<SessionCreatePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Create session called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    // Make sure the uuids are valid
    let patient_uuid = Uuid::parse_str(&payload.patient_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;
    let professional_uuid = Uuid::parse_str(&payload.professional_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;
    let session_type_uuid = payload.session_type_id
        .map(|uid| Uuid::parse_str(&uid).map_err(|_| AppError::Internal("Invalid UUID string".into())))
        .transpose()?;

    let session = Session { id: None, patient_id: patient_uuid, professional_id: professional_uuid, session_type_id: session_type_uuid, session_status: SessionStatus::from_id(payload.session_status_id.unwrap_or(1)).unwrap_or_default(), session_date: payload.session_date, videocall_url: payload.videocall_url, notes: payload.notes, completed: false, session_duration: payload.session_duration, created_at: None };

    use_cases
        .create(&session)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(SessionCreateResponse { success:true }),
    ))
}