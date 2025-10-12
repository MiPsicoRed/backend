use std::sync::Arc;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    adapters::http::routes::{blog_post::BlogPostResponse, Validateable}, app_error::{AppError, AppResult}, use_cases::blog_post::BlogPostUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
pub struct BlogPostReadSingleQuery {
    #[param(example = "insert-blog-post-uuid")]
    blog_post_id: String,
}

impl Validateable for BlogPostReadSingleQuery {
    fn valid(&self) -> bool {
        !self.blog_post_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BlogPostReadSingleResponse {
    data: BlogPostResponse,
    success: bool,
}

#[utoipa::path(get, path = "/api/blog_post/single", 
    params(BlogPostReadSingleQuery),
    responses( 
        (status = 200, description = "Data retrieved correctly", body = BlogPostReadSingleResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Session",
    summary = "Retrieves data of a single blog post",
    description = "\n\n**Required:**  Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn read_single_blog_post(
    State(use_cases): State<Arc<BlogPostUseCases>>,
    Query(params): Query<BlogPostReadSingleQuery>,
) -> AppResult<impl IntoResponse> {
    info!("Read single blog post called");
    if !params.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let blog_post_uuid = Uuid::parse_str(&params.blog_post_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let session = use_cases
        .read_single(blog_post_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(BlogPostReadSingleResponse { success:true , data: session.into()}),
    ))
}