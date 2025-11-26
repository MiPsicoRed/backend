use std::sync::Arc;

use axum::{Extension, Json, extract::{Query, State}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{AuthUser, Validateable, session::SessionResponse}, app_error::{AppError, AppResult}, entities::{professional::Professional, user::Role}, use_cases::{professional::ProfessionalUseCases, session::SessionUseCases}
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
    description = "\n\n**Required:**  Verified Email + Admin Role or Professional + requesting professional_id"
)]
#[instrument(skip(professional_use_cases, session_use_cases))]
pub async fn read_professional_sessions(
    Extension(auth_user): Extension<AuthUser>,
    State(professional_use_cases): State<Arc<ProfessionalUseCases>>,
    State(session_use_cases): State<Arc<SessionUseCases>>,
    Query(params): Query<SessionReadProfessionalQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read professional sessions called");
    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let professional_uuid = Uuid::parse_str(&params.professional_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let professional = professional_use_cases
        .read_single(&professional_uuid)
        .await?;

    let is_authorized = authorized(&auth_user, &professional);
    if !is_authorized {
        return Err(AppError::Unauthorized(
            String::from("You don't have permission for this endpoint")
        ));
    }

    let sessions = session_use_cases
        .read_professional(&professional_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(SessionReadProfessionalResponse { success:true , data: sessions.into_iter().map(Into::into).collect() }),
    ))
}

fn authorized(auth_user: &AuthUser, professional: &Professional) -> bool {
    let requesting_role = Role::from_id(auth_user.role_id).unwrap_or_default();
    
    // Check authorization
    match requesting_role {
        Role::Admin => true,
        Role::Professional => {
            professional.user_id
                .as_ref()
                .map(|id| id.to_string() == auth_user.user_id)
                .unwrap_or(false) // Don't allow if no user_id specified
        },
        Role::Patient => false,
    }
}