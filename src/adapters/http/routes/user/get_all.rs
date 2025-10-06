use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use tracing::instrument;

use crate::{app_error::AppResult, entities::user::User, use_cases::user::UserUseCases};

#[derive(Debug, Serialize)]
pub struct GetAllUsersResponse {
    success: bool,
    data: Vec<User>,
}

#[instrument(skip(user_use_cases))]
pub async fn get_all_users(
    State(user_use_cases): State<Arc<UserUseCases>>,
) -> AppResult<impl IntoResponse> {
    let users = user_use_cases.get_all_users().await?;

    Ok((
        StatusCode::OK,
        Json(GetAllUsersResponse {
            success: true,
            data: users,
        }),
    ))
}
