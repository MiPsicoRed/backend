use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{AuthUser, Validateable, patient::PatientResponse}, app_error::{AppError, AppResult}, entities::{professional::Professional, user::Role}, use_cases::{patient::PatientUseCases, professional::ProfessionalUseCases}
};

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct PatientReadByProfessionalQuery {
    #[param(example = "insert-professional-uuid")]
    professional_id: String,
}

impl Validateable for PatientReadByProfessionalQuery {
    fn valid(&self) -> bool {
        !self.professional_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PatientReadByProfessionalResponse {
    data: Vec<PatientResponse>,
    success: bool,
}

#[utoipa::path(get, path = "/api/patient/professional", 
    params(PatientReadByProfessionalQuery),
    responses( 
        (status = 200, description = "Data retrieved correctly", body = PatientReadByProfessionalResponse),
        (status = 400, description = "Invalid payload"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Patient",
    summary = "Retrieves data of all patients of the requested professional",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role and requesting professional_id"
)]
#[instrument(skip(patient_use_cases, professional_use_cases))]
pub async fn read_patients_by_professional(
    Extension(auth_user): Extension<AuthUser>,
    State(patient_use_cases): State<Arc<PatientUseCases>>,
    State(professional_use_cases): State<Arc<ProfessionalUseCases>>,
    Query(params): Query<PatientReadByProfessionalQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read professional patients called");
    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let professional_uuid = Uuid::parse_str(&params.professional_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;
    let professional = professional_use_cases.read_single(&professional_uuid).await?;

    let is_authorized = authorized(&auth_user, &professional);
    if !is_authorized {
        return Err(AppError::Unauthorized(
            String::from("You don't have permission for this endpoint")
        ));
    }

    let patients = patient_use_cases
        .read_by_professional(&professional_uuid)
        .await?;
    
    Ok((
        StatusCode::OK,
        Json(PatientReadByProfessionalResponse { success:true , data: patients.into_iter().map(Into::into).collect() }),
    ))
}

fn authorized(auth_user: &AuthUser, professional: &Professional) -> bool {
    let requesting_role = Role::from_id(auth_user.role_id).unwrap_or_default();
    
    // Check authorization
    match requesting_role {
        Role::Admin => true,
        Role::Professional =>{
            auth_user.user_id
                .parse::<Uuid>()
                .ok()
                .is_some_and(|id| Some(id) == professional.user_id)
        },
        _ => false,
    }
}