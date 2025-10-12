use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::routes::Validateable, app_error::{AppError, AppResult}, use_cases::blog_post::BlogPostUseCases
};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct BlogPostDeletePayload {
    blog_post_id: String,
}

impl Validateable for BlogPostDeletePayload {
    fn valid(&self) -> bool {
        !self.blog_post_id.is_empty()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BlogPostDeleteResponse {
    success: bool,
}

#[utoipa::path(delete, path = "/api/blog_post/delete", 
    responses( 
        (status = 200, description = "Deleted", body = BlogPostDeleteResponse),
        (status = 400, description = "Invalid payload"),
        (status = 500, description = "Internal server error or database error")
    ),
    security(
        ("bearer_auth" = [])  
    ),
    tag = "Blog Post",
    summary = "Deletes a blog post",
    description = "\n\n**Required:** Verified Email + Admin Role"
)]
#[instrument(skip(use_cases))]
pub async fn delete_blog_post(
    State(use_cases): State<Arc<BlogPostUseCases>>,
    Json(payload): Json<BlogPostDeletePayload>,
) -> AppResult<impl IntoResponse> {
    info!("Delete blog post called");

    if !payload.valid() {
        return AppResult::Err(AppError::InvalidPayload);
    }

    let blog_post_uuid = Uuid::parse_str(&payload.blog_post_id).map_err(|_| AppError::Internal("Invalid UUID string".into()))?;

    use_cases
        .delete(blog_post_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(BlogPostDeleteResponse { success:true }),
    ))
}
