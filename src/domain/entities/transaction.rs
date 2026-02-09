use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Expired,
}

impl ToString for TransactionStatus {
    fn to_string(&self) -> String {
        match self {
            TransactionStatus::Pending => "pending".to_string(),
            TransactionStatus::Completed => "completed".to_string(),
            TransactionStatus::Failed => "failed".to_string(),
            TransactionStatus::Expired => "expired".to_string(),
        }
    }
}

impl From<String> for TransactionStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "completed" => TransactionStatus::Completed,
            "failed" => TransactionStatus::Failed,
            "expired" => TransactionStatus::Expired,
            _ => TransactionStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub payment_intent_id: Option<String>,
    pub session_id: String,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Transaction {
    pub fn new(session_id: String, amount: Option<i64>, currency: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            payment_intent_id: None,
            session_id,
            amount,
            currency,
            status: TransactionStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn complete(&mut self, payment_intent_id: String) {
        self.status = TransactionStatus::Completed;
        self.payment_intent_id = Some(payment_intent_id);
        self.updated_at = Utc::now();
    }

    pub fn fail(&mut self) {
        self.status = TransactionStatus::Failed;
        self.updated_at = Utc::now();
    }
}
