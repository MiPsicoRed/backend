use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{ professional::ProfessionalResponse, AuthUser, Validateable}, app_error::{AppError, AppResult}, entities::{user::Role}, use_cases::{professional::ProfessionalUseCases}
};

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct ProfessionalReadByUserQuery {
    #[param(example = "insert-user-uuid")]
    user_id: String,
}

impl Validateable for ProfessionalReadByUserQuery {
    fn valid(&self) -> bool {
        !self.user_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfessionalReadByUserResponse {
    data: ProfessionalResponse,
    success: bool,
}

#[utoipa::path(get, path = "/api/professional/user", 
    params(ProfessionalReadByUserQuery),
    responses( 
        (status = 200, description = "Data retrieved correctly", body = ProfessionalReadByUserResponse),
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
pub async fn read_professional_by_user(
    Extension(auth_user): Extension<AuthUser>,
    State(use_cases): State<Arc<ProfessionalUseCases>>,
    Query(params): Query<ProfessionalReadByUserQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read single professional called");
    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let user_uuid = Uuid::parse_str(&params.user_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let is_authorized = authorized(&auth_user, &user_uuid);
    if !is_authorized {
        return Err(AppError::Unauthorized(
            String::from("You don't have permission for this endpoint")
        ));
    }

    let professional = use_cases
        .read_by_user(&user_uuid)
        .await?;
    
    Ok((
        StatusCode::OK,
        Json(ProfessionalReadByUserResponse { success:true , data: professional.into()}),
    ))
}

fn authorized(auth_user: &AuthUser, user_id: &Uuid) -> bool {
    let requesting_role = Role::from_id(auth_user.role_id).unwrap_or_default();
    
    // Check authorization
    match requesting_role {
        Role::Admin => true,
        Role::Professional => {
            user_id.to_string() == auth_user.user_id
        },
        _ => false,
    }
}