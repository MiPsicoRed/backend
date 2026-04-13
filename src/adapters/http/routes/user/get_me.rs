use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use uuid::Uuid;

use crate::{
    adapters::{http::{app_state::AppState, routes::AuthUser}},
    app_error::AppError,
};
use super::UserResponse;

pub async fn get_me(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = auth_user.user_id;
    let uuid = Uuid::parse_str(&user_id).map_err(|_| AppError::InvalidPayload)?;
    
    let user = state.user_use_cases.get_user_by_id(&uuid).await?;
    let onboarding_info = state.user_use_cases.get_onboarding_info(&uuid).await?;
    
    let mut response: UserResponse = user.into();
    response.onboarding_info = onboarding_info;
    
    Ok((StatusCode::OK, Json(response)))
}
