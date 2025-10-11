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
        // session types
        routes::session_type::create::create_session_type,
        routes::session_type::delete::delete_session_type,
        routes::session_type::read_all::read_all_session_types,
        routes::session_type::read_single::read_single_session_type,
        routes::session_type::update::update_session_type,
    ),
    components(
        schemas(
            // user
            routes::user::get_all::GetAllUsersResponse,
            routes::user::login::LoginResponse,
            routes::user::register::RegisterResponse,
            // user_token
            routes::user_token::generate::GenerateResponse,
            routes::user_token::verify::VerifyResponse,
            routes::user_token::validate::ValidateResponse,
            // patient
            routes::patient::create::CreateResponse,
            routes::patient::delete::DeleteResponse,
            routes::patient::read_all::ReadAllResponse,
            routes::patient::read_single::ReadSingleResponse,
            routes::patient::update::UpdateResponse,
            // session types
            routes::session_type::create::CreateResponse,
            routes::session_type::delete::DeleteResponse,
            routes::session_type::read_all::ReadAllResponse,
            routes::session_type::read_single::ReadSingleResponse,
            routes::session_type::update::UpdateResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "User", description = "User endpoints"),
        (name = "User Token", description = "User Token endpoints"),
        (name = "Patient", description = "Patient endpoints"),
        (name = "Session Type", description = "Session Type endpoints"),
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
