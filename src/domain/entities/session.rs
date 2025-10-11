use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug)]
pub struct Session {
    pub id: Option<Uuid>, // we option this so we can use the same type for update and create but aside that on_create it should never be None
    pub patient_id: Uuid,
    pub professional_id: Uuid,
    pub session_type_id: Option<Uuid>,
    pub session_status: SessionStatus,
    pub session_date: Option<chrono::NaiveDateTime>,
    pub videocall_url: Option<String>,
    pub notes: Option<String>,
    pub completed: bool,
    pub session_duration: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Default)]
pub enum SessionStatus {
    #[default]
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}

impl Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SessionStatus::Scheduled => write!(f, "Scheduled"),
            SessionStatus::InProgress => write!(f, "InProgress"),
            SessionStatus::Completed => write!(f, "Completed"),
            SessionStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl SessionStatus {
    pub const ALL: &'static [Self] = &[
        Self::Scheduled,
        Self::InProgress,
        Self::Completed,
        Self::Cancelled,
    ];

    pub fn to_id(&self) -> i32 {
        match self {
            SessionStatus::Scheduled => 1,
            SessionStatus::InProgress => 2,
            SessionStatus::Completed => 3,
            SessionStatus::Cancelled => 4,
        }
    }

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(SessionStatus::Scheduled),
            2 => Some(SessionStatus::InProgress),
            3 => Some(SessionStatus::Completed),
            4 => Some(SessionStatus::Cancelled),
            _ => None,
        }
    }
}
