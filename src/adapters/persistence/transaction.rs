use async_trait::async_trait;
use sqlx::query;


use crate::{

    app_error::{AppError, AppResult},
    application::use_cases::payment::TransactionPersistence,
    domain::entities::transaction::{Transaction, TransactionStatus},
};

use super::PostgresPersistence;

#[async_trait]
impl TransactionPersistence for PostgresPersistence {
    async fn create(&self, transaction: &Transaction) -> AppResult<Transaction> {
        query!(
            r#"
            INSERT INTO transactions (id, payment_intent_id, session_id, amount, currency, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            transaction.id,
            transaction.payment_intent_id,
            transaction.session_id,
            transaction.amount,
            transaction.currency,
            transaction.status.to_string(),
            transaction.created_at.naive_utc(),
            transaction.updated_at.naive_utc()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create transaction: {:?}", e);
            AppError::Database(e)
        })?;

        Ok(transaction.clone())
    }

    async fn update(&self, transaction: &Transaction) -> AppResult<Transaction> {
        query!(
            r#"
            UPDATE transactions
            SET payment_intent_id = $1, status = $2, updated_at = $3
            WHERE id = $4
            "#,
            transaction.payment_intent_id,
            transaction.status.to_string(),
            transaction.updated_at.naive_utc(),
            transaction.id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update transaction: {:?}", e);
            AppError::Database(e)
        })?;

        Ok(transaction.clone())
    }

    async fn get_by_session_id(&self, session_id: &str) -> AppResult<Transaction> {
        let rec = query!(
            r#"
            SELECT id, payment_intent_id, session_id, amount, currency, status, created_at, updated_at
            FROM transactions
            WHERE session_id = $1
            "#,
            session_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch transaction: {:?}", e);
            AppError::Database(e)
        })?;

        match rec {
            Some(row) => Ok(Transaction {
                id: row.id,
                payment_intent_id: row.payment_intent_id,
                session_id: row.session_id,
                amount: row.amount,
                currency: row.currency,
                status: TransactionStatus::from(row.status),
                created_at: row.created_at.expect("created_at cannot be null").and_utc(),
                updated_at: row.updated_at.expect("updated_at cannot be null").and_utc(),
            }),
            None => Err(AppError::NotFound(format!("Transaction with session_id {} not found", session_id))),
        }
    }
}
