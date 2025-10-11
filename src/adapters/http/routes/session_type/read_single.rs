use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{session_type::SessionTypeResponse, Validateable}, app_error::{AppError, AppResult}, use_cases::session_type::SessionTypeUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct SessionTypeReadSingleQuery {
    #[param(example = "insert-session-type-uuid")]
    session_type_id: String,
}

impl Validateable for SessionTypeReadSingleQuery {
    fn valid(&self) -> bool {
        !self.session_type_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionTypeReadSingleResponse {
    data: SessionTypeResponse,
    success: bool,
}

#[utoipa::path(get, path = "/api/session_type/single", 
    params(SessionTypeReadSingleQuery),
    responses( 
        (status = 200, description = "Data retrieved correctly", body = SessionTypeReadSingleResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Session Type",
    summary = "Retrieves data of a single session type",
    description = "\n\n**Required:** Verified Email"
)]
#[instrument(skip(use_cases))]
pub async fn read_single_session_type(
    State(use_cases): State<Arc<SessionTypeUseCases>>,
    Query(params): Query<SessionTypeReadSingleQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read single session type called");

    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let session_type_uuid = Uuid::parse_str(&params.session_type_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let session_type = use_cases
        .read_single(session_type_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(SessionTypeReadSingleResponse { success:true , data: session_type.into()}),
    ))
}