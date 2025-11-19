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
pub struct SessionReadProfessionalQuery {
    #[param(example = "insert-professional-uuid")]
    professional_id: String,
}

impl Validateable for SessionReadProfessionalQuery {
    fn valid(&self) -> bool {
        !self.professional_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionReadProfessionalResponse {
    data: Vec<SessionResponse>,
    success: bool,
}

#[utoipa::path(get, path = "/api/session/professional", 
    params(SessionReadProfessionalQuery),
    responses( 
        (status = 200, description = "Data retrieved correctly", body = SessionReadProfessionalResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Session",
    summary = "Retrieves data of all sessions of a given professional",
    description = "\n\n**Required:**  Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn read_professional_sessions(
    State(use_cases): State<Arc<SessionUseCases>>,
    Query(params): Query<SessionReadProfessionalQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read professional sessions called");
    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let professional_uuid = Uuid::parse_str(&params.professional_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let sessions = use_cases
        .read_professional(&professional_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(SessionReadProfessionalResponse { success:true , data: sessions.into_iter().map(Into::into).collect() }),
    ))
}