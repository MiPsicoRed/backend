use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    app_error::{AppError, AppResult},
    entities::blog_post::{BlogPost, BlogPostStatus},
    use_cases::blog_post::BlogPostPersistence,
};

// Session struct as stored in the db.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct BlogPostDb {
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

impl From<BlogPostDb> for BlogPost {
    fn from(blog_post_db: BlogPostDb) -> Self {
        BlogPost {
            id: Some(blog_post_db.id),
            author_id: Some(blog_post_db.author_id),
            blog_post_status: BlogPostStatus::from_id(blog_post_db.blog_post_status_id)
                .unwrap_or_default(),
            title: blog_post_db.title,
            slug: blog_post_db.slug,
            summary: blog_post_db.summary,
            content: blog_post_db.content,
            tags: blog_post_db.tags,
            featured_image: blog_post_db.featured_image,
            reading_time_minutes: blog_post_db.reading_time_minutes,
            view_count: blog_post_db.view_count,
            created_at: blog_post_db.created_at,
        }
    }
}

#[async_trait]
impl BlogPostPersistence for PostgresPersistence {
    async fn create(&self, blog_post: &BlogPost) -> AppResult<()> {
        let uuid = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO blog_posts (id, author_id, blog_post_status_id, title, slug, summary, content, tags, featured_image, reading_time_minutes, view_count) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            uuid,
            blog_post.author_id,
            blog_post.blog_post_status.to_id(),
            blog_post.title,
            blog_post.slug,
            blog_post.summary,
            blog_post.content,
            blog_post.tags.as_deref(),
            blog_post.featured_image,
            blog_post.reading_time_minutes,
            blog_post.view_count
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn read_all(&self) -> AppResult<Vec<BlogPost>> {
        sqlx::query_as!(
            BlogPostDb,
            r#"
                SELECT id, author_id, blog_post_status_id, title, slug, summary, content, tags, featured_image, reading_time_minutes, view_count, created_at
                FROM blog_posts
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(|blog_posts| blog_posts.into_iter().map(BlogPost::from).collect())
    }

    async fn read_single(&self, id: Uuid) -> AppResult<BlogPost> {
        sqlx::query_as!(
            BlogPostDb,
            r#"
                SELECT id, author_id, blog_post_status_id, title, slug, summary, content, tags, featured_image, reading_time_minutes, view_count, created_at
                FROM blog_posts 
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)
        .map(BlogPost::from)
    }

    async fn update(&self, blog_post: &BlogPost) -> AppResult<()> {
        sqlx::query!(
            "UPDATE blog_posts 
                SET blog_post_status_id = $2, title = $3, slug = $4, summary = $5, content = $6, tags = $7, featured_image = $8, reading_time_minutes = $9, view_count = $10
                WHERE id = $1",
            blog_post.id,
            blog_post.blog_post_status.to_id(),
            blog_post.title,
            blog_post.slug,
            blog_post.summary,
            blog_post.content,
            blog_post.tags.as_deref(),
            blog_post.featured_image,
            blog_post.reading_time_minutes,
            blog_post.view_count
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query!("DELETE FROM blog_posts WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }
}
