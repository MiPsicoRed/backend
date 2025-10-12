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
            professional_specialization::{
                create::create_professional_specialization,
                delete::delete_professional_specialization,
                read_all::read_all_professional_specializations,
                read_single::read_single_professional_specialization,
                update::update_professional_specialization,
            },
            require_admin, require_professional_or_admin, require_role_middleware,
            verified_middleware,
        },
    },
    entities::professional_specialization::ProfessionalSpecialization,
};

pub mod create;
pub mod delete;
pub mod read_all;
pub mod read_single;
pub mod update;

#[derive(Debug, Serialize, ToSchema)]
struct ProfessionalSpecializationResponse {
    pub id: Uuid,
    pub professional_id: Uuid,
    pub name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<ProfessionalSpecialization> for ProfessionalSpecializationResponse {
    fn from(professional_specialization: ProfessionalSpecialization) -> Self {
        ProfessionalSpecializationResponse {
            id: professional_specialization.id.unwrap(), // This should never panic as this should never be null when responding
            professional_id: professional_specialization.professional_id.unwrap(), // This should never panic as this should never be null when responding
            name: professional_specialization.name,
            created_at: professional_specialization.created_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/create", // Required: Verified Email + Admin/Professional Role
            post(create_professional_specialization)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .route(
            "/delete", // Required: Verified Email + Admin/Professional Role
            delete(delete_professional_specialization)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .route(
            "/all", // Required: Verified Email + Admin Role
            get(read_all_professional_specializations)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_admin()),
        )
        .route(
            "/single", // Required: Verified Email + Admin/Professional Role
            get(read_single_professional_specialization)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .route(
            "/update", // Required: Verified Email + Admin/Professional Role
            patch(update_professional_specialization)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .layer(middleware::from_fn(verified_middleware))
        .layer(middleware::from_fn(auth_middleware))
}
