use axum::{Extension, Json, Router, extract::{Path, State}, middleware, routing::{get, post, delete}};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    adapters::http::{app_state::AppState, routes::{AuthUser, auth_middleware, verified_middleware}},
    app_error::{AppError, AppResult},
    entities::mood_log::MoodLog,
};

#[derive(Deserialize)]
pub struct CreateMoodLogRequest {
    pub mood_score: i32,
    pub note: Option<String>,
    pub shared_with_professional: bool,
}

pub async fn create_mood_log(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<CreateMoodLogRequest>,
) -> AppResult<Json<MoodLog>> {
    let user_uuid = Uuid::parse_str(&auth_user.user_id).unwrap();
    let patient = state.patient_use_cases.read_by_user(&user_uuid).await?;
    let patient_id = patient.id.unwrap();

    let mood_log = MoodLog {
        id: Uuid::new_v4(),
        patient_id,
        mood_score: payload.mood_score,
        note: payload.note,
        shared_with_professional: payload.shared_with_professional,
        created_at: None,
    };

    state.mood_log_use_cases.create(&mood_log).await?;
    Ok(Json(mood_log))
}

pub async fn get_my_mood_logs(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> AppResult<Json<Vec<MoodLog>>> {
    let user_uuid = Uuid::parse_str(&auth_user.user_id).unwrap();
    let patient = state.patient_use_cases.read_by_user(&user_uuid).await?;
    
    let logs = state.mood_log_use_cases.read_by_patient(&patient.id.unwrap()).await?;
    Ok(Json(logs))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_mood_log))
        .route("/mine", get(get_my_mood_logs))
        .layer(middleware::from_fn(verified_middleware))
        .layer(middleware::from_fn(auth_middleware))
}
