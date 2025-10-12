use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug)]
pub struct BlogPost {
    pub id: Option<Uuid>, // we option this so we can use the same type for update and create but aside that on_create it should never be None
    pub author_id: Option<Uuid>, // we option this so we don't need to pass it for update, as once created we can't modify the user
    pub blog_post_status: BlogPostStatus,
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

#[derive(Debug, Default)]
pub enum BlogPostStatus {
    #[default]
    Draft,
    Published,
    Archived,
}

impl Display for BlogPostStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            BlogPostStatus::Draft => write!(f, "Draft"),
            BlogPostStatus::Published => write!(f, "Published"),
            BlogPostStatus::Archived => write!(f, "Archived"),
        }
    }
}

impl BlogPostStatus {
    pub const ALL: &'static [Self] = &[Self::Draft, Self::Published, Self::Archived];

    pub fn to_id(&self) -> i32 {
        match self {
            BlogPostStatus::Draft => 1,
            BlogPostStatus::Published => 2,
            BlogPostStatus::Archived => 3,
        }
    }

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(BlogPostStatus::Draft),
            2 => Some(BlogPostStatus::Published),
            3 => Some(BlogPostStatus::Archived),
            _ => None,
        }
    }
}
