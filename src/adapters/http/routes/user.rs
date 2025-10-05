use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::{
    adapters::{
        http::{
            app_state::AppState,
            routes::{Validateable, auth_middleware},
        },
        persistence::user::UserDb,
    },
    app_error::{AppError, AppResult},
    use_cases::user::UserUseCases,
};

pub fn router() -> Router<AppState> {
    let public_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));

    let protected_routes = Router::new()
        .route("/all", get(get_all_users))
        .layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}

#[derive(Debug, Clone, Deserialize)]
struct RegisterPayload {
    username: String,
    email: String,
    password: SecretString,
}

impl Validateable for RegisterPayload {
    fn valid(&self) -> bool {
        !self.email.is_empty()
            && !self.password.expose_secret().is_empty()
            && !self.username.is_empty()
    }
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    success: bool,
}

/// Creates a new user based on the submitted credentials.
#[instrument(skip(user_use_cases))]
async fn register(
    State(user_use_cases): State<Arc<UserUseCases>>,
    Json(payload): Json<RegisterPayload>,
) -> AppResult<impl IntoResponse> {
    info!("Register user called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    user_use_cases
        .add(&payload.username, &payload.email, &payload.password)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse { success: true }),
    ))
}

#[derive(Debug, Clone, Deserialize)]
struct LoginPayload {
    username: String,
    password: SecretString,
}

impl Validateable for LoginPayload {
    fn valid(&self) -> bool {
        !self.username.is_empty() && !self.password.expose_secret().is_empty()
    }
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    jwt: String,
    success: bool,
}

/// Attempts to login as the specified user
#[instrument(skip(user_use_cases))]
async fn login(
    State(user_use_cases): State<Arc<UserUseCases>>,
    Json(payload): Json<LoginPayload>,
) -> AppResult<impl IntoResponse> {
    info!("Register user called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let jwt = user_use_cases
        .login(&payload.username, &payload.password)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(LoginResponse { success: true, jwt }),
    ))
}

#[derive(Debug, Serialize)]
struct GetAllUsersResponse {
    success: bool,
    data: Vec<UserDb>, // TODO: Should we return this user struct/the other one why? :C
}

#[instrument(skip(user_use_cases))]
async fn get_all_users(
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
