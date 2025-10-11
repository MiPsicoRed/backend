use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

use crate::{
    adapters::http::routes::{session::SessionResponse}, app_error::AppResult, use_cases::session::SessionUseCases
};

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionReadAllResponse {
    data: Vec<SessionResponse>,
    success: bool,
}

#[utoipa::path(get, path = "/api/session/all", 
    responses( 
        (status = 200, description = "Data retrieved correctly", body = SessionReadAllResponse),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Session",
    summary = "Returns all sessions with their info",
    description = "\n\n**Required:** Verified Email + Admin Role"
)]
#[instrument(skip(use_cases))]
pub async fn read_all_sessions(
    State(use_cases): State<Arc<SessionUseCases>>,
) -> AppResult<impl IntoResponse> {
    info!("Read all sessions called");

    let sessions = use_cases
        .read_all()
        .await?;

    Ok((
        StatusCode::OK,
        Json(SessionReadAllResponse { success:true, data: sessions.into_iter().map(Into::into).collect() }),
    ))
}
