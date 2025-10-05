use std::sync::Arc;

use axum::extract::FromRef;

use crate::{
    infra::config::AppConfig,
    use_cases::{user::UserUseCases, user_token::UserTokenUseCases},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_use_cases: Arc<UserUseCases>,
    pub user_token_use_cases: Arc<UserTokenUseCases>,
}

impl FromRef<AppState> for Arc<UserUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<UserTokenUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_token_use_cases.clone()
    }
}
