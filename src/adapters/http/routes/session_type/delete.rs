use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, use_cases::session_type::SessionTypeUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct DeletePayload {
    session_type_id: String,
}

impl Validateable for DeletePayload {
    fn valid(&self) -> bool {
        !self.session_type_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteResponse {
    success: bool,
}

#[utoipa::path(delete, path = "/api/session_type/delete", 
    responses( 
        (status = 200, description = "Deleted", body = DeleteResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Session Type",
    summary = "Deletes a session type",
    description = "\n\n**Required:** Verified Email + Admin Role"
)]
#[instrument(skip(use_cases))]
pub async fn delete_session_type(
    State(use_cases): State<Arc<SessionTypeUseCases>>,
    Json(payload): Json<DeletePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Delete session type called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let session_type_uuid = Uuid::parse_str(&payload.session_type_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    use_cases
        .delete(session_type_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(DeleteResponse { success:true }),
    ))
}
