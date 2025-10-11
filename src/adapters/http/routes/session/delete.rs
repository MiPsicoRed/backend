use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, use_cases::session::SessionUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct SessionDeletePayload {
    session_id: String,
}

impl Validateable for SessionDeletePayload {
    fn valid(&self) -> bool {
        !self.session_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionDeleteResponse {
    success: bool,
}

#[utoipa::path(delete, path = "/api/session/delete", 
    responses( 
        (status = 200, description = "Deleted", body = SessionDeleteResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Session",
    summary = "Deletes a session",
    description = "\n\n**Required:** Verified Email + Admin Role"
)]
#[instrument(skip(use_cases))]
pub async fn delete_session(
    State(use_cases): State<Arc<SessionUseCases>>,
    Json(payload): Json<SessionDeletePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Delete session called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let session_uuid = Uuid::parse_str(&payload.session_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    use_cases
        .delete(session_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(SessionDeleteResponse { success:true }),
    ))
}
