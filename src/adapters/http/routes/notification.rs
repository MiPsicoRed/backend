use axum::{Extension, Json, Router, extract::{Path, State}, middleware, routing::{get, patch}};
use uuid::Uuid;

use crate::{
    adapters::http::{app_state::AppState, routes::{AuthUser, auth_middleware, verified_middleware}},
    app_error::AppResult,
    entities::notification::Notification,
};

pub async fn get_my_notifications(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> AppResult<Json<Vec<Notification>>> {
    let user_id = Uuid::parse_str(&auth_user.user_id).unwrap();
    let notifications = state.notification_use_cases.read_for_user(&user_id).await?;
    Ok(Json(notifications))
}

pub async fn mark_as_read(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<()>> {
    state.notification_use_cases.mark_as_read(&id).await?;
    Ok(Json(()))
}

pub async fn mark_all_as_read(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> AppResult<Json<()>> {
    let user_id = Uuid::parse_str(&auth_user.user_id).unwrap();
    state.notification_use_cases.mark_all_as_read(&user_id).await?;
    Ok(Json(()))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_my_notifications))
        .route("/read-all", patch(mark_all_as_read))
        .route("/:id/read", patch(mark_as_read))
        .layer(middleware::from_fn(verified_middleware))
        .layer(middleware::from_fn(auth_middleware))
}
