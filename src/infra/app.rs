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

    let cors = CorsLayer::new()
        .allow_origin(
            app_state
                .config
                .base_frontend_url
                .parse::<http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([http::Method::POST, http::Method::GET])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    let jwt_service_ext = app_state.user_use_cases.jwt_service.clone();

    Router::new()
        .merge(
            utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", super::api_doc::ApiDoc::openapi()),
        )
        .nest("/api", adapters::http::routes::router())
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
