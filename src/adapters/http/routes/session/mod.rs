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
            session::{
                create::create_session, delete::delete_session, read_all::read_all_sessions,
                read_single::read_single_session, update::update_session,
            },
            verified_middleware,
        },
    },
    entities::session::Session,
};

pub mod create;
pub mod delete;
pub mod read_all;
pub mod read_single;
pub mod update;

#[derive(Debug, Serialize, ToSchema)]
struct SessionResponse {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub professional_id: Uuid,
    pub session_type_id: Option<Uuid>,
    pub session_status_id: i32,
    pub session_date: Option<chrono::NaiveDateTime>,
    pub videocall_url: Option<String>,
    pub notes: Option<String>,
    pub completed: bool,
    pub session_duration: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<Session> for SessionResponse {
    fn from(session: Session) -> Self {
        SessionResponse {
            id: session.id.unwrap(), // This should never panic as this should never be null when responding
            patient_id: session.patient_id,
            professional_id: session.professional_id,
            session_type_id: session.session_type_id,
            session_status_id: session.session_status.to_id(),
            session_date: session.session_date,
            videocall_url: session.videocall_url,
            notes: session.notes,
            completed: session.completed,
            session_duration: session.session_duration,
            created_at: session.created_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/create", // Required: Verified Email + Admin/Professional Role
            post(create_session)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .route(
            "/delete", // Required: Verified Email + Admin Role
            delete(delete_session)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_admin()),
        )
        .route(
            "/all", // Required: Verified Email + Admin Role
            get(read_all_sessions)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_admin()),
        )
        .route(
            "/single", // Required: Verified Email + Admin/Professional Role
            get(read_single_session)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .route(
            "/update", // Required: Verified Email + Admin/Professional Role
            patch(update_session)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .layer(middleware::from_fn(verified_middleware))
        .layer(middleware::from_fn(auth_middleware))
}
