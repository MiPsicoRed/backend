use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

use crate::{
    adapters::http::routes::{AuthUser, Validateable}, app_error::{AppError, AppResult}, entities::user::Role, use_cases::user_token::UserTokenUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct GeneratePayload {
    user_id: String,
}

impl Validateable for GeneratePayload {
    fn valid(&self) -> bool {
        !self.user_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GenerateResponse {
    success: bool,
}

#[utoipa::path(post, path = "/api/user_token/generate", 
    responses( 
        (status = 200, description = "Success", body = GenerateResponse),
        (status = 400, description = "Invalid payload"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error or database error")
    ), 
    security(
        ("bearer_auth" = [])  
    ),
    tag = "User Token",
    summary = "Generates a new verification token for the given user_id and sends them a confirmation email",
    description = "\n\n**Required:** Admin Role OR Generating for requesting user_id"
)]
#[instrument(skip(user_token_use_cases))]
pub async fn generate_token(
    Extension(auth_user): Extension<AuthUser>,
    State(user_token_use_cases): State<Arc<UserTokenUseCases>>,
    Json(payload): Json<GeneratePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Generate user token called");
    let is_authorized = authorized(&auth_user, &payload);
    if !is_authorized {
        return Err(AppError::Unauthorized(
            String::from("You don't have permission for this endpoint")
        ));
    }

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    user_token_use_cases
        .generate_token_and_send_mail(&payload.user_id)
        .await?;

    Ok((
        StatusCode::OK,
        Json(GenerateResponse {
            success: true
        }),
    ))
}

fn authorized(auth_user: &AuthUser, payload: &GeneratePayload) -> bool {
    let requesting_role = Role::from_id(auth_user.role_id).unwrap_or_default();
    
    // Check authorization
    match requesting_role {
        Role::Admin => true,
        Role::Patient => {
            payload.user_id == auth_user.user_id
        }
        _ => false
    }
}