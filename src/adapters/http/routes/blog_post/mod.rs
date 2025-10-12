use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    adapters::http::{
        app_state::AppState,
        routes::{
            auth_middleware,
            blog_post::{
                create::create_blog_post, delete::delete_blog_post, read_all::read_all_blog_posts,
                read_single::read_single_blog_post, update::update_blog_post,
            },
            require_admin, require_professional_or_admin, require_role_middleware,
            verified_middleware,
        },
    },
    entities::blog_post::BlogPost,
};

pub mod create;
pub mod delete;
pub mod read_all;
pub mod read_single;
pub mod update;

#[derive(Debug, Serialize, ToSchema)]
struct BlogPostResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    pub blog_post_status_id: i32,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub featured_image: Option<String>,
    pub reading_time_minutes: Option<i32>,
    pub view_count: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl From<BlogPost> for BlogPostResponse {
    fn from(blog_post: BlogPost) -> Self {
        BlogPostResponse {
            id: blog_post.id.unwrap(), // This should never panic as this should never be null when responding
            author_id: blog_post.author_id.unwrap(), // This should never panic as this should never be null when responding
            blog_post_status_id: blog_post.blog_post_status.to_id(),
            title: blog_post.title,
            slug: blog_post.slug,
            summary: blog_post.summary,
            content: blog_post.content,
            tags: blog_post.tags,
            featured_image: blog_post.featured_image,
            reading_time_minutes: blog_post.reading_time_minutes,
            view_count: blog_post.view_count,
            created_at: blog_post.created_at,
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/create", // Required: Verified Email + Admin/Professional Role
            post(create_blog_post)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .route(
            "/delete", // Required: Verified Email + Admin Role
            delete(delete_blog_post)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_admin()),
        )
        .route(
            "/all", // Required: Verified Email + Admin Role
            get(read_all_blog_posts)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_admin()),
        )
        .route(
            "/single", // Required: Verified Email + Admin/Professional Role
            get(read_single_blog_post)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .route(
            "/update", // Required: Verified Email + Admin/Professional Role
            patch(update_blog_post)
                .route_layer(middleware::from_fn(require_role_middleware))
                .route_layer(require_professional_or_admin()),
        )
        .layer(middleware::from_fn(verified_middleware))
        .layer(middleware::from_fn(auth_middleware))
}
