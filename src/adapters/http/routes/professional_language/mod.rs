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
            professional_language::{
                create::create_professional_language, delete::delete_professional_language,
                read_all::read_all_professional_languages,
                read_single::read_single_professional_language,
                update::update_professional_language,
            },
            require_admin, require_professional_or_admin, require_role_middleware,
            verified_middleware,
        },
    },
    entities::professional_language::ProfessionalLanguage,
};

pub mod create;
pub mod delete;
pub mod read_all;
pub mod read_single;
pub mod update;

#[derive(Debug, Serialize, ToSchema)]
struct ProfessionalLanguageResponse {
    pub id: Uuid,
    pub professional_id: Uuid,
    pub name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<ProfessionalLanguage> for ProfessionalLanguageResponse {
    fn from(professional_language: ProfessionalLanguage) -> Self {
        ProfessionalLanguageResponse {
            id: professional_language.id.unwrap(), // This should never panic as this should never be null when responding
            professional_id: professional_language.professional_id.unwrap(), // This should never panic as this should never be null when responding
            name: professional_language.name,
            created_at: professional_language.created_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/create", // Required: Verified Email + Admin/Professional Role
            post(create_professional_language)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .route(
            "/delete", // Required: Verified Email + Admin/Professional Role
            delete(delete_professional_language)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .route(
            "/all", // Required: Verified Email + Admin Role
            get(read_all_professional_languages)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_admin()),
        )
        .route(
            "/single", // Required: Verified Email + Admin/Professional Role
            get(read_single_professional_language)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .route(
            "/update", // Required: Verified Email + Admin/Professional Role
            patch(update_professional_language)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .layer(middleware::from_fn(verified_middleware))
        .layer(middleware::from_fn(auth_middleware))
}
