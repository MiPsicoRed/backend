use std::fmt::Display;

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Email {
    pub id: Uuid,
    pub from_mail: String,
    pub to_mail: String,
    pub mail_subject: String,
    pub mail_body: String,
    pub email_kind: EmailKind,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub enum EmailKind {
    #[default]
    Verification,
}

impl Display for EmailKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            EmailKind::Verification => write!(f, "Verification"),
        }
    }
}

impl EmailKind {
    pub fn to_id(self) -> i32 {
        match self {
            EmailKind::Verification => 1,
        }
    }

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(EmailKind::Verification),
            _ => None,
        }
    }
}
