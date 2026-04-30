use std::sync::Arc;
use axum::{extract::{Path, State}, Extension, Json, response::IntoResponse};
use serde::Serialize;
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::AuthUser,
    app_error::{AppError, AppResult},
    use_cases::session::SessionUseCases,
};

#[derive(Debug, Serialize, ToSchema)]
pub struct VideoCallResponse {
    url: String,
}

#[utoipa::path(get, path = "/api/session/{id}/videocall", 
    responses( 
        (status = 200, description = "Success", body = VideoCallResponse),
        (status = 400, description = "Too early to join or missing data"),
        (status = 404, description = "Session not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = String, Path, description = "Session ID")
    ),
    security(
        ("bearer_auth" = [])  
    ), 
    tag = "Session",
    summary = "Gets or generates the videocall URL for a session",
    description = "Available starting 5 minutes before the session starts."
)]
#[instrument(skip(use_cases, _auth_user))]
pub async fn get_videocall_url(
    State(use_cases): State<Arc<SessionUseCases>>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    info!("Get videocall URL called for session: {}", id);

    let session_uuid = Uuid::parse_str(&id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;
    
    let user_id = Uuid::parse_str(&_auth_user.user_id)
        .map_err(|_| AppError::Internal("Invalid User UUID in token".into()))?;

    let url = use_cases
        .get_videocall_url(&session_uuid, &user_id)
        .await?;

    Ok(Json(VideoCallResponse { url }))
}
