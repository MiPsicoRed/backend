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
            professional::{
                create::create_professional, delete::delete_professional,
                read_all::read_all_professionals, read_single::read_single_professional,
                update::update_professional,
            },
            require_admin, require_professional_or_admin, require_role_middleware,
            verified_middleware,
        },
    },
    entities::professional::Professional,
};

pub mod create;
pub mod delete;
pub mod read_all;
pub mod read_single;
pub mod update;

#[derive(Debug, Serialize, ToSchema)]
struct ProfessionalResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub gender: i32,
    pub birthdate: Option<chrono::NaiveDate>,
    pub license_number: Option<String>,
    pub bio: Option<String>,
    pub education: Option<String>,
    pub experience_years: Option<i32>,
    pub hourly_rate: Option<f32>,
    pub accepts_insurance: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<Professional> for ProfessionalResponse {
    fn from(professional: Professional) -> Self {
        ProfessionalResponse {
            id: professional.id.unwrap(), // This should never panic as this should never be null when responding
            user_id: professional.user_id.unwrap(), // This should never panic as this should never be null when responding
            gender: professional.gender.to_id(),
            birthdate: professional.birthdate,
            license_number: professional.license_number,
            bio: professional.bio,
            education: professional.education,
            experience_years: professional.experience_years,
            hourly_rate: professional.hourly_rate,
            accepts_insurance: professional.accepts_insurance,
            created_at: professional.created_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/create", // Required: Verified Email + Admin/Professional Role
            post(create_professional)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .route(
            "/delete", // Required: Verified Email + Admin Role
            delete(delete_professional)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_admin()),
        )
        .route(
            "/all", // Required: Verified Email + Admin Role
            get(read_all_professionals)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_admin()),
        )
        .route("/single", get(read_single_professional)) // Required: Verified Email + Admin Role or Professional Role + requesting user_id
        .route(
            "/update", // Required: Verified Email + Admin/Professional Role
            patch(update_professional)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .layer(middleware::from_fn(verified_middleware))
        .layer(middleware::from_fn(auth_middleware))
}
