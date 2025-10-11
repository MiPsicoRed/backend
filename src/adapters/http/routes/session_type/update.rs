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
pub struct UpdatePayload {
    id: String,
    name: String,
}

impl Validateable for UpdatePayload {
    fn valid(&self) -> bool {
        !self.name.is_empty() && !self.id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UpdateResponse {
    success: bool,
}

#[utoipa::path(patch, path = "/api/session_type/update", 
    responses( 
        (status = 200, description = "Updated", body = UpdateResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Session Type",
    summary = "Updates a session type",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn update_session_type(
    State(use_cases): State<Arc<SessionTypeUseCases>>,
    Json(payload): Json<UpdatePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Update session type called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let id = Uuid::parse_str(&payload.id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    use_cases
        .update(id, &payload.name)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(UpdateResponse { success:true }),
    ))
}
