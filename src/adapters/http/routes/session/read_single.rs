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
pub struct SessionReadSingleQuery {
    #[param(example = "insert-session-uuid")]
    session_id: String,
}

impl Validateable for SessionReadSingleQuery {
    fn valid(&self) -> bool {
        !self.session_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionReadSingleResponse {
    data: SessionResponse,
    success: bool,
}

#[utoipa::path(get, path = "/api/session/single", 
    params(SessionReadSingleQuery),
    responses( 
        (status = 200, description = "Data retrieved correctly", body = SessionReadSingleResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Session",
    summary = "Retrieves data of a single session",
    description = "\n\n**Required:**  Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn read_single_session(
    State(use_cases): State<Arc<SessionUseCases>>,
    Query(params): Query<SessionReadSingleQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read single session called");
    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let session_uuid = Uuid::parse_str(&params.session_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let session = use_cases
        .read_single(session_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(SessionReadSingleResponse { success:true , data: session.into()}),
    ))
}