use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{ professional::ProfessionalResponse, AuthUser, Validateable}, app_error::{AppError, AppResult}, entities::{professional::Professional, user::Role}, use_cases::{professional::ProfessionalUseCases}
};

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct ProfessionalReadSingleQuery {
    #[param(example = "insert-professional-uuid")]
    professional_id: String,
}

impl Validateable for ProfessionalReadSingleQuery {
    fn valid(&self) -> bool {
        !self.professional_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalReadSingleResponse {
    data: ProfessionalResponse,
    success: bool,
}

#[utoipa::path(get, path = "/api/professional/single", 
    params(ProfessionalReadSingleQuery),
    responses( 
        (status = 200, description = "Data retrieved correctly", body = ProfessionalReadSingleResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Professional",
    summary = "Retrieves data of a single professional",
    description = "\n\n**Required:**  Verified Email + Admin Role or Professional + requesting user_id"
)]
#[instrument(skip(use_cases))]
pub async fn read_single_professional(
    Extension(auth_user): Extension<AuthUser>,
    State(use_cases): State<Arc<ProfessionalUseCases>>,
    Query(params): Query<ProfessionalReadSingleQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read single professional called");
    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let professional_uuid = Uuid::parse_str(&params.professional_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let professional = use_cases
        .read_single(professional_uuid)
        .await?;

    let is_authorized = authorized(&auth_user, &professional);
    if !is_authorized {
        return Err(AppError::Unauthorized(
            String::from("You don't have permission for this endpoint")
        ));
    }


    Ok((
        StatusCode::OK,
        Json(ProfessionalReadSingleResponse { success:true , data: professional.into()}),
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
                .unwrap_or(false)
        },
        _ => false,
    }
}