use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;

use crate::{
    adapters::http::routes::{blog_post::BlogPostResponse}, app_error::AppResult, use_cases::blog_post::BlogPostUseCases
};

#[derive(Debug, Serialize, ToSchema)]
pub struct BlogPostReadAllResponse {
    data: Vec<BlogPostResponse>,
    success: bool,
}

#[utoipa::path(get, path = "/api/blog_post/all", 
    responses( 
        (status = 200, description = "Data retrieved correctly", body = BlogPostReadAllResponse),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Blog Post",
    summary = "Returns all blog posts with their info",
    description = "\n\n**Required:** Verified Email + Admin Role"
)]
#[instrument(skip(use_cases))]
pub async fn read_all_blog_posts(
    State(use_cases): State<Arc<BlogPostUseCases>>,
) -> AppResult<impl IntoResponse> {
    info!("Read all blog posts called");

    let blog_posts = use_cases
        .read_all()
        .await?;

    Ok((
        StatusCode::OK,
        Json(BlogPostReadAllResponse { success:true, data: blog_posts.into_iter().map(Into::into).collect() }),
    ))
}
