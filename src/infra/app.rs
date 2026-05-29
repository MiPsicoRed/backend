use axum::{Extension, Router, http};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    adapters::{self, http::app_state::AppState},
    infra::setup::init_tracing,
};

pub fn create_app(app_state: AppState) -> Router {
    init_tracing();

    let frontend_url = app_state.config.base_frontend_url.trim_end_matches('/').to_string();
    let mut origins = vec![
        frontend_url.parse::<http::HeaderValue>().unwrap(),
    ];
    
    // Also allow www variant if it's a production domain
    if frontend_url.contains("mipsicored.com") && !frontend_url.contains("www.") {
        let www_url = frontend_url.replace("https://", "https://www.");
        if let Ok(val) = www_url.parse::<http::HeaderValue>() {
            origins.push(val);
        }
    }

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            http::Method::POST,
            http::Method::GET,
            http::Method::PATCH,
            http::Method::DELETE,
            http::Method::OPTIONS,
        ])
        .allow_headers([
            CONTENT_TYPE,
            AUTHORIZATION,
            http::header::ACCEPT,
            http::header::ORIGIN,
            http::HeaderName::from_static("x-requested-with"),
        ])
        .allow_credentials(true);

    let jwt_service_ext = app_state.user_use_cases.jwt_service.clone();

    Router::new()
        .merge(
            utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", super::api_doc::ApiDoc::openapi()),
        )
        .nest("/api", adapters::http::routes::router())
        .nest_service(
            "/api/uploads",
            tower_http::services::ServeDir::new("uploads"),
        )
        .with_state(app_state)
        .layer(Extension(jwt_service_ext))
        .layer(cors)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
                let request_id = Uuid::new_v4();
                tracing::info_span!(
                    "http-request",
                    method = %request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                    request_id = %request_id
                )
            }),
        )
}
