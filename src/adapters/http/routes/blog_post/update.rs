use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, entities::blog_post::{BlogPost, BlogPostStatus}, use_cases::blog_post::BlogPostUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct BlogPostUpdatePayload {
    id: String,
    blog_post_status_id: i32,
    title: String,
    slug: String,
    summary: Option<String>,
    content: String,
    tags: Option<Vec<String>>,
    featured_image: Option<String>,
    reading_time_minutes: Option<i32>,
    view_count: Option<i32>,
}

impl Validateable for BlogPostUpdatePayload {
    fn valid(&self) -> bool {
        !self.id.is_empty() && !self.title.is_empty() && !self.slug.is_empty() && !self.content.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BlogPostUpdateResponse {
    success: bool,
}

#[utoipa::path(patch, path = "/api/blog_post/update", 
    responses( 
        (status = 200, description = "Updated", body = BlogPostUpdateResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Blog Post",
    summary = "Updates a blog post",
    description = "\n\n**Required:** Verified Email + Admin/Professional Role"
)]
#[instrument(skip(use_cases))]
pub async fn update_blog_post(
    State(use_cases): State<Arc<BlogPostUseCases>>,
    Json(payload): Json<BlogPostUpdatePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Update blog post called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let id = Uuid::parse_str(&payload.id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    let blog_post = BlogPost { id: Some(id), author_id: None, blog_post_status: BlogPostStatus::from_id(payload.blog_post_status_id).unwrap_or_default(), title: payload.title, slug: payload.slug, summary: payload.summary, content: payload.content, tags: payload.tags, featured_image: payload.featured_image, reading_time_minutes: payload.reading_time_minutes, view_count: payload.view_count, created_at: None };

    use_cases
        .update(&blog_post)
        .await?;

    Ok((
        StatusCode::OK,
        Json(BlogPostUpdateResponse { success:true }),
    ))
}
