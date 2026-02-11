use std::sync::Arc;

use axum::extract::FromRef;

use crate::{
    infra::config::AppConfig,
    use_cases::{
        blog_post::BlogPostUseCases,
        patient::PatientUseCases,
        payment::PaymentUseCases,
        professional::ProfessionalUseCases,
        professional_language::ProfessionalLanguageUseCases,
        professional_specialization::ProfessionalSpecializationUseCases, session::SessionUseCases,
        session_type::SessionTypeUseCases, user::UserUseCases, user_token::UserTokenUseCases,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub user_use_cases: Arc<UserUseCases>,
    pub user_token_use_cases: Arc<UserTokenUseCases>,
    pub patient_use_cases: Arc<PatientUseCases>,
    pub session_type_use_cases: Arc<SessionTypeUseCases>,
    pub session_use_cases: Arc<SessionUseCases>,
    pub professional_use_cases: Arc<ProfessionalUseCases>,
    pub professional_languages_use_cases: Arc<ProfessionalLanguageUseCases>,
    pub professional_specializations_use_cases: Arc<ProfessionalSpecializationUseCases>,
    pub blog_post_use_cases: Arc<BlogPostUseCases>,
    pub payment_use_cases: Arc<PaymentUseCases>,
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

impl FromRef<AppState> for Arc<SessionUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.session_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<ProfessionalUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.professional_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<ProfessionalLanguageUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.professional_languages_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<ProfessionalSpecializationUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.professional_specializations_use_cases.clone()
    }
}


impl FromRef<AppState> for Arc<BlogPostUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.blog_post_use_cases.clone()
    }
}

impl FromRef<AppState> for Arc<PaymentUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.payment_use_cases.clone()
    }
}

