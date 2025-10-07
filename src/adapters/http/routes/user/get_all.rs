use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use tracing::instrument;
use utoipa::ToSchema;

use crate::{
    adapters::http::routes::user::UserResponse, app_error::AppResult, use_cases::user::UserUseCases,
};

#[derive(Debug, Serialize, ToSchema)]
pub struct GetAllUsersResponse {
    success: bool,
    data: Vec<UserResponse>,
}

#[utoipa::path(get, path = "/api/user/all", 
    responses( 
        (status = 200, description = "Success", body = GetAllUsersResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error or database error")
    ), 
    security(
        ("bearer_auth" = [])  
    ),
    tag = "User",
    summary = "Get all users"
)]
#[instrument(skip(user_use_cases))]
pub async fn get_all_users(
    State(user_use_cases): State<Arc<UserUseCases>>,
) -> AppResult<impl IntoResponse> {
    let users = user_use_cases.get_all_users().await?;

    Ok((
        StatusCode::OK,
        Json(GetAllUsersResponse {
            success: true,
            data: users.into_iter().map(Into::into).collect(),
        }),
    ))
}
