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
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "User", description = "User endpoints"),
        (name = "User Token", description = "User Token endpoints"),
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
                        .build(),
                ),
            )
        }
    }
}
