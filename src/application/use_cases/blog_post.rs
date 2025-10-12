use std::sync::Arc;

use async_trait::async_trait;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{app_error::AppResult, entities::blog_post::BlogPost};

#[async_trait]
pub trait BlogPostPersistence: Send + Sync {
    async fn create(&self, blog_post: &BlogPost) -> AppResult<()>;

    async fn read_all(&self) -> AppResult<Vec<BlogPost>>;

    async fn read_single(&self, id: Uuid) -> AppResult<BlogPost>;

    async fn update(&self, blog_post: &BlogPost) -> AppResult<()>;

    async fn delete(&self, id: Uuid) -> AppResult<()>;
}

#[derive(Clone)]
pub struct BlogPostUseCases {
    persistence: Arc<dyn BlogPostPersistence>,
}

impl BlogPostUseCases {
    pub fn new(persistence: Arc<dyn BlogPostPersistence>) -> Self {
        Self { persistence }
    }

    #[instrument(skip(self))]
    pub async fn create(&self, blog_post: &BlogPost) -> AppResult<()> {
        info!("Attempting create blog post...");

        self.persistence.create(blog_post).await?;

        info!("Blog post created.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn read_all(&self) -> AppResult<Vec<BlogPost>> {
        self.persistence.read_all().await
    }

    #[instrument(skip(self))]
    pub async fn read_single(&self, id: Uuid) -> AppResult<BlogPost> {
        self.persistence.read_single(id).await
    }

    #[instrument(skip(self))]
    pub async fn update(&self, blog_post: &BlogPost) -> AppResult<()> {
        info!("Attempting update blog post...");

        self.persistence.update(blog_post).await?;

        info!("Blog post updated.");

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: Uuid) -> AppResult<()> {
        info!("Attempting delete blog post...");

        self.persistence.delete(id).await?;

        info!("Blog post deleted.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use crate::{app_error::AppError, entities::blog_post::BlogPostStatus};

    use super::*;

    struct MockBlogPostPersistence;

    #[async_trait]
    impl BlogPostPersistence for MockBlogPostPersistence {
        async fn create(&self, blog_post: &BlogPost) -> AppResult<()> {
            if blog_post.id.is_some() {
                return Err(AppError::Internal(
                    "blog_post id must be None when creating".into(),
                ));
            }

            Ok(())
        }

        async fn read_all(&self) -> AppResult<Vec<BlogPost>> {
            Ok(vec![])
        }

        async fn read_single(&self, _id: Uuid) -> AppResult<BlogPost> {
            Ok(BlogPost {
                id: Some(Uuid::new_v4()),
                author_id: Some(Uuid::new_v4()),
                blog_post_status: BlogPostStatus::Draft,
                title: String::from("My awesome blog post"),
                slug: String::from("mapost"),
                summary: None,
                content: String::from("Blog post content"),
                tags: None,
                featured_image: None,
                reading_time_minutes: Some(30),
                view_count: Some(0),
                created_at: None,
            })
        }

        async fn update(&self, blog_post: &BlogPost) -> AppResult<()> {
            assert!(blog_post.id.is_some());

            Ok(())
        }

        async fn delete(&self, _id: Uuid) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn create_works() {
        let use_cases = BlogPostUseCases::new(Arc::new(MockBlogPostPersistence));

        let result = use_cases
            .create(&BlogPost {
                id: None,
                author_id: Some(Uuid::new_v4()),
                blog_post_status: BlogPostStatus::Draft,
                title: String::from("My awesome blog post"),
                slug: String::from("mapost"),
                summary: None,
                content: String::from("Blog post content"),
                tags: None,
                featured_image: None,
                reading_time_minutes: Some(30),
                view_count: Some(0),
                created_at: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn create_with_id_fails() {
        let use_cases = BlogPostUseCases::new(Arc::new(MockBlogPostPersistence));

        let result = use_cases
            .create(&BlogPost {
                id: Some(Uuid::new_v4()),
                author_id: Some(Uuid::new_v4()),
                blog_post_status: BlogPostStatus::Draft,
                title: String::from("My awesome blog post"),
                slug: String::from("mapost"),
                summary: None,
                content: String::from("Blog post content"),
                tags: None,
                featured_image: None,
                reading_time_minutes: Some(30),
                view_count: Some(0),
                created_at: None,
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn read_all_works() {
        let use_cases = BlogPostUseCases::new(Arc::new(MockBlogPostPersistence));

        let result = use_cases.read_all().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn read_single_works() {
        let use_cases = BlogPostUseCases::new(Arc::new(MockBlogPostPersistence));

        let result = use_cases.read_single(Uuid::new_v4()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_works() {
        let use_cases = BlogPostUseCases::new(Arc::new(MockBlogPostPersistence));

        let result = use_cases
            .update(&BlogPost {
                id: Some(Uuid::new_v4()),
                author_id: Some(Uuid::new_v4()),
                blog_post_status: BlogPostStatus::Draft,
                title: String::from("My awesome blog post"),
                slug: String::from("mapost"),
                summary: None,
                content: String::from("Blog post content"),
                tags: None,
                featured_image: None,
                reading_time_minutes: Some(30),
                view_count: Some(0),
                created_at: None,
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_works() {
        let use_cases = BlogPostUseCases::new(Arc::new(MockBlogPostPersistence));

        let result = use_cases.delete(Uuid::new_v4()).await;

        assert!(result.is_ok());
    }
}
