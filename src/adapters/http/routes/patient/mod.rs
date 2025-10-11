use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::{
        app_state::AppState,
        routes::{
            auth_middleware,
            patient::{
                create::create_patient, delete::delete_patient, read_all::read_all_patients,
                read_single::read_single_patient, update::update_patient,
            },
            require_admin, require_role_middleware, verified_middleware,
        },
    },
    entities::patient::Patient,
};

pub mod create;
pub mod delete;
pub mod read_all;
pub mod read_single;
pub mod update;

#[derive(Debug, Serialize, ToSchema)]
struct PatientResponse {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub gender: i32,
    pub sexual_orientation: i32,
    pub birthdate: Option<chrono::NaiveDate>,
    pub phone: String,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub insurance_policy_number: Option<String>,
    pub medical_history: Option<String>,
    pub current_medications: Option<String>,
    pub allergies: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<Patient> for PatientResponse {
    fn from(patient: Patient) -> Self {
        PatientResponse {
            id: patient.id.unwrap(), // This should never panic as this should never be null when responding
            user_id: patient.user_id,
            gender: patient.gender.to_id(),
            sexual_orientation: patient.sexual_orientation.to_id(),
            birthdate: patient.birthdate,
            phone: patient.phone,
            emergency_contact_name: patient.emergency_contact_name,
            emergency_contact_phone: patient.emergency_contact_phone,
            insurance_policy_number: patient.insurance_policy_number,
            medical_history: patient.medical_history,
            current_medications: patient.current_medications,
            allergies: patient.allergies,
            created_at: patient.created_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_patient)) // Required: Verified Email + Admin/Professional Role OR Creating for requesting user_id
        .route(
            "/delete",
            delete(delete_patient) // Require verified + admin
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_admin()),
        )
        .route(
            "/all",
            get(read_all_patients) // Require verified + admin
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_admin()),
        )
        .route("/single", get(read_single_patient)) // Required: Verified Email + Admin/Professional Role or requesting user_id
        .route("/update", patch(update_patient)) // Only auth + mail verified required
        .layer(middleware::from_fn(verified_middleware))
        .layer(middleware::from_fn(auth_middleware))
}
