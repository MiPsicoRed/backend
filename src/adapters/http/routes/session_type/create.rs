use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

use crate::{
    adapters::http::routes::{AuthUser, Validateable}, app_error::{AppError, AppResult}, use_cases::session_type::SessionTypeUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreatePayload {
    name: String
}

impl Validateable for CreatePayload {
    fn valid(&self) -> bool {
        !self.name.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateResponse {
    success: bool,
}

#[utoipa::path(post, path = "/api/session_type/create", 
    responses( 
        (status = 201, description = "Created", body = CreateResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ), 
    tag = "Session Type",
    summary = "Creates a new session type",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn create_session_type(
    Extension(auth_user): Extension<AuthUser>,
    State(use_cases): State<Arc<SessionTypeUseCases>>,
    Json(payload): Json<CreatePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Create session type called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    use_cases
        .create(&payload.name)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateResponse { success:true }),
    ))
}
