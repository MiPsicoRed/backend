use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{patient::PatientResponse, AuthUser, Validateable}, app_error::{AppError, AppResult}, entities::{user::Role}, use_cases::patient::PatientUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct PatientReadByUserQuery {
    #[param(example = "insert-user-uuid")]
    user_id: String,
}

impl Validateable for PatientReadByUserQuery {
    fn valid(&self) -> bool {
        !self.user_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PatientReadByUserResponse {
    data: PatientResponse,
    success: bool,
}

#[utoipa::path(get, path = "/api/patient/user", 
    params(PatientReadByUserQuery),
    responses( 
        (status = 200, description = "Data retrieved correctly", body = PatientReadByUserResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Patient",
    summary = "Retrieves data of a single patient",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role or requesting user_id"
)]
#[instrument(skip(use_cases))]
pub async fn read_patient_by_user(
    Extension(auth_user): Extension<AuthUser>,
    State(use_cases): State<Arc<PatientUseCases>>,
    Query(params): Query<PatientReadByUserQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read single patient called");
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

    let patient = use_cases
        .read_by_user(&user_uuid)
        .await?;
    
    Ok((
        StatusCode::OK,
        Json(PatientReadByUserResponse { success:true , data: patient.into()}),
    ))
}

fn authorized(auth_user: &AuthUser, user_id: &Uuid) -> bool {
    let requesting_role = Role::from_id(auth_user.role_id).unwrap_or_default();
    
    // Check authorization
    match requesting_role {
        Role::Admin | Role::Professional => true,
        Role::Patient => {
            user_id.to_string() == auth_user.user_id
        }
    }
}