use crate::adapters::http::routes;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        //user
        routes::user::get_all::get_all_users,
        routes::user::login::login,
        routes::user::register::register,
        routes::user::onboard::onboard_user,
        //user_token
        routes::user_token::generate::generate_token,
        routes::user_token::verify::verify,
        routes::user_token::validate::validate_token,
        //patient
        routes::patient::create::create_patient,
        routes::patient::delete::delete_patient,
        routes::patient::read_all::read_all_patients,
        routes::patient::read_single::read_single_patient,
        routes::patient::update::update_patient,
        routes::patient::read_by_user::read_patient_by_user,
        routes::patient::read_by_professional::read_patients_by_professional,
        // session types
        routes::session_type::create::create_session_type,
        routes::session_type::delete::delete_session_type,
        routes::session_type::read_all::read_all_session_types,
        routes::session_type::read_single::read_single_session_type,
        routes::session_type::update::update_session_type,
        // sessions
        routes::session::create::create_session,
        routes::session::delete::delete_session,
        routes::session::read_all::read_all_sessions,
        routes::session::read_single::read_single_session,
        routes::session::update::update_session,
        routes::session::professional::read_professional_sessions,
        routes::session::patient::read_patient_sessions,
        // professionals
        routes::professional::create::create_professional,
        routes::professional::delete::delete_professional,
        routes::professional::read_all::read_all_professionals,
        routes::professional::read_single::read_single_professional,
        routes::professional::update::update_professional,
        routes::professional::read_by_user::read_professional_by_user,
        routes::professional::selector::professionals_selector,
        // professional languages
        routes::professional_language::create::create_professional_language,
        routes::professional_language::delete::delete_professional_language,
        routes::professional_language::read_all::read_all_professional_languages,
        routes::professional_language::read_single::read_single_professional_language,
        routes::professional_language::update::update_professional_language,
        // professional specializations
        routes::professional_specialization::create::create_professional_specialization,
        routes::professional_specialization::delete::delete_professional_specialization,
        routes::professional_specialization::read_all::read_all_professional_specializations,
        routes::professional_specialization::read_single::read_single_professional_specialization,
        routes::professional_specialization::update::update_professional_specialization,
        // blog posts
        routes::blog_post::create::create_blog_post,
        routes::blog_post::delete::delete_blog_post,
        routes::blog_post::read_all::read_all_blog_posts,
        routes::blog_post::read_single::read_single_blog_post,
        routes::blog_post::update::update_blog_post,
    ),
    components(
        schemas(
            // user
            routes::user::get_all::GetAllUsersResponse,
            routes::user::login::LoginResponse,
            routes::user::register::RegisterResponse,
            routes::user::onboard::OnboardResponse,
            // user_token
            routes::user_token::generate::GenerateResponse,
            routes::user_token::verify::VerifyResponse,
            routes::user_token::validate::ValidateResponse,
            // patient
            routes::patient::create::PatientCreateResponse,
            routes::patient::delete::PatientDeleteResponse,
            routes::patient::read_all::PatientReadAllResponse,
            routes::patient::read_single::PatientReadSingleResponse,
            routes::patient::update::PatientUpdateResponse,
            routes::patient::read_by_user::PatientReadByUserResponse,
            routes::patient::read_by_professional::PatientReadByProfessionalResponse,
            // session types
            routes::session_type::create::SessionTypeCreateResponse,
            routes::session_type::delete::SessionTypeDeleteResponse,
            routes::session_type::read_all::SessionTypeReadAllResponse,
            routes::session_type::read_single::SessionTypeReadSingleResponse,
            routes::session_type::update::SessionTypeUpdateResponse,
            // sessions
            routes::session::create::SessionCreateResponse,
            routes::session::delete::SessionDeleteResponse,
            routes::session::read_all::SessionReadAllResponse,
            routes::session::read_single::SessionReadSingleResponse,
            routes::session::update::SessionUpdateResponse,
            routes::session::professional::SessionReadProfessionalResponse,
            routes::session::patient::SessionReadPatientResponse,
            // professionals
            routes::professional::create::ProfessionalCreateResponse,
            routes::professional::delete::ProfessionalDeleteResponse,
            routes::professional::read_all::ProfessionalReadAllResponse,
            routes::professional::read_single::ProfessionalReadSingleResponse,
            routes::professional::update::ProfessionalUpdateResponse,
            routes::professional::read_by_user::ProfessionalReadByUserResponse,
            routes::professional::selector::ProfessionalSelectorResponse,
            // professional languages
            routes::professional_language::create::ProfessionalLanguageCreateResponse,
            routes::professional_language::delete::ProfessionalLanguageDeleteResponse,
            routes::professional_language::read_all::ProfessionalLanguageReadAllResponse,
            routes::professional_language::read_single::ProfessionalLanguageReadSingleResponse,
            routes::professional_language::update::ProfessionalLanguageUpdateResponse,
            // professional specializations
            routes::professional_specialization::create::ProfessionalSpecializationCreateResponse,
            routes::professional_specialization::delete::ProfessionalSpecializationDeleteResponse,
            routes::professional_specialization::read_all::ProfessionalSpecializationReadAllResponse,
            routes::professional_specialization::read_single::ProfessionalSpecializationReadSingleResponse,
            routes::professional_specialization::update::ProfessionalSpecializationUpdateResponse,
            // blog posts
            routes::blog_post::create::BlogPostCreateResponse,
            routes::blog_post::delete::BlogPostDeleteResponse,
            routes::blog_post::read_all::BlogPostReadAllResponse,
            routes::blog_post::read_single::BlogPostReadSingleResponse,
            routes::blog_post::update::BlogPostUpdateResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "User", description = "User endpoints"),
        (name = "User Token", description = "User Token endpoints"),
        (name = "Patient", description = "Patient endpoints"),
        (name = "Session Type", description = "Session Type endpoints"),
        (name = "Session", description = "Session endpoints"),
        (name = "Professional", description = "Professional endpoints"),
        (name = "Professional Language", description = "Professional languages endpoints"),
        (name = "Professional Specialization", description = "Professional specializations endpoints"),
        (name = "Blog Post", description = "Blog Post endpoints"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some(
                            "JWT Bearer token for authentication.\n\n\
                            Some endpoints require verified email or specific roles. \
                            Check individual endpoint descriptions for requirements.",
                        ))
                        .build(),
                ),
            )
        }
    }
}
