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
            auth_middleware, require_admin, require_professional_or_admin, require_role_middleware,
            session_type::{
                create::create_session_type, delete::delete_session_type,
                read_all::read_all_session_types, read_single::read_single_session_type,
                update::update_session_type,
            },
            verified_middleware,
        },
    },
    entities::session_type::SessionType,
};

pub mod create;
pub mod delete;
pub mod read_all;
pub mod read_single;
pub mod update;

#[derive(Debug, Serialize, ToSchema)]
struct SessionTypeResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<SessionType> for SessionTypeResponse {
    fn from(session_type: SessionType) -> Self {
        SessionTypeResponse {
            id: session_type.id,
            name: session_type.name,
            created_at: session_type.created_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/create", // Required: Verified Email + Admin/Professional Role
            post(create_session_type)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .route(
            "/delete", // Required: Verified Email + Admin Role
            delete(delete_session_type)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_admin()),
        )
        .route("/all", get(read_all_session_types)) // Required: Verified Email
        .route("/single", get(read_single_session_type)) // Required: Verified Email
        .route(
            "/update", // Required: Verified Email + Admin/Professional Role
            patch(update_session_type)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .layer(middleware::from_fn(verified_middleware))
        .layer(middleware::from_fn(auth_middleware))
}
