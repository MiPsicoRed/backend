use crate::{
    adapters::http::app_state::AppState,
    infra::{
        argon2_password_hasher, config::AppConfig, email_service, jwt_service, polar_service,
        postgres_persistence,
    },
    use_cases::{
        blog_post::BlogPostUseCases,
        patient::PatientUseCases,
        professional::ProfessionalUseCases,
        professional_language::ProfessionalLanguageUseCases,
        professional_specialization::ProfessionalSpecializationUseCases,
        session::SessionUseCases,
        session_type::SessionTypeUseCases,
        user::{UserJwtService, UserUseCases},
        user_token::{UserTokenJwtService, UserTokenUseCases},
    },
};
use std::fs::File;
use std::sync::Arc;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub async fn init_app_state() -> anyhow::Result<AppState> {
    let config = Arc::new(AppConfig::from_env());

    let postgres_arc = Arc::new(postgres_persistence().await?);
    let jwt_service = Arc::new(jwt_service(Arc::clone(&config)));
    let polar_service = polar_service(Arc::clone(&config));
    let email_service = email_service(Arc::clone(&config));
    let argon_hasher = argon2_password_hasher();

    let user_use_cases = UserUseCases::new(
        jwt_service.clone() as Arc<dyn UserJwtService>,
        Arc::new(polar_service),
        Arc::new(argon_hasher),
        postgres_arc.clone(),
    );

    let user_token_use_cases = UserTokenUseCases::new(
        jwt_service as Arc<dyn UserTokenJwtService>,
        Arc::new(email_service),
        postgres_arc.clone(),
    );

    let patient_use_cases = PatientUseCases::new(postgres_arc.clone());

    let session_type_use_cases = SessionTypeUseCases::new(postgres_arc.clone());

    let session_use_cases = SessionUseCases::new(postgres_arc.clone());

    let professional_use_cases = ProfessionalUseCases::new(postgres_arc.clone());

    let professional_languages_use_cases = ProfessionalLanguageUseCases::new(postgres_arc.clone());

    let professional_specializations_use_cases =
        ProfessionalSpecializationUseCases::new(postgres_arc.clone());

    let blog_post_use_cases = BlogPostUseCases::new(postgres_arc.clone());

    Ok(AppState {
        config,
        user_use_cases: Arc::new(user_use_cases),
        user_token_use_cases: Arc::new(user_token_use_cases),
        patient_use_cases: Arc::new(patient_use_cases),
        session_type_use_cases: Arc::new(session_type_use_cases),
        session_use_cases: Arc::new(session_use_cases),
        professional_use_cases: Arc::new(professional_use_cases),
        professional_languages_use_cases: Arc::new(professional_languages_use_cases),
        professional_specializations_use_cases: Arc::new(professional_specializations_use_cases),
        blog_post_use_cases: Arc::new(blog_post_use_cases),
    })
}

pub fn init_tracing() {
    // let filter = EnvFilter::try_from_default_env()
    //     .unwrap_or_else(|_| "axum_trainer=debug,tower_http=debug".into());

    // Console (pretty logs)
    let console_layer = fmt::layer()
        .with_target(false) // don’t show target (module path)
        .with_level(true) // show log level
        .pretty(); // human-friendly, with colors

    // File (structured JSON logs)
    let file = File::create("app.log").expect("cannot create log file");
    let json_layer = fmt::layer()
        .json()
        .with_writer(file)
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        // .with(filter)
        .with(console_layer)
        .with(json_layer)
        .try_init()
        .ok();
}
