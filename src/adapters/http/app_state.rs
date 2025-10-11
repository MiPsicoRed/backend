use std::sync::Arc;

use axum::extract::FromRef;

use crate::{
    infra::config::AppConfig,
    use_cases::{
        patient::PatientUseCases, session_type::SessionTypeUseCases, user::UserUseCases,
        user_token::UserTokenUseCases,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_use_cases: Arc<UserUseCases>,
    pub user_token_use_cases: Arc<UserTokenUseCases>,
    pub patient_use_cases: Arc<PatientUseCases>,
    pub session_type_use_cases: Arc<SessionTypeUseCases>,
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

impl FromRef<AppState> for Arc<PatientUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.patient_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<SessionTypeUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.session_type_use_cases.clone()
    }
}
