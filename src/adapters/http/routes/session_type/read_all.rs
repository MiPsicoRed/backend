use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

use crate::{
    adapters::http::routes::{ session_type::SessionTypeResponse}, app_error::AppResult, use_cases::session_type::SessionTypeUseCases
};

#[derive(Debug, Serialize, ToSchema)]
pub struct ReadAllResponse {
    data: Vec<SessionTypeResponse>,
    success: bool,
}

#[utoipa::path(get, path = "/api/session_type/all", 
    responses( 
        (status = 200, description = "Data retrieved correctly", body = ReadAllResponse),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Session Type",
    summary = "Returns all session types with their info",
    description = "\n\n**Required:** Verified Email"
)]
#[instrument(skip(use_cases))]
pub async fn read_all_session_types(
    State(use_cases): State<Arc<SessionTypeUseCases>>,
) -> AppResult<impl IntoResponse> {
    info!("Read all session types called");

    let session_types = use_cases
        .read_all()
        .await?;

    Ok((
        StatusCode::OK,
        Json(ReadAllResponse { success:true, data: session_types.into_iter().map(Into::into).collect() }),
    ))
}
