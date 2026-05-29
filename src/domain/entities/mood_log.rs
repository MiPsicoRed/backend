use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MoodLog {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub mood_score: i32,
    pub note: Option<String>,
    pub shared_with_professional: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
