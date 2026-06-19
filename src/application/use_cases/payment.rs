use async_trait::async_trait;

use chrono::Utc;
use std::sync::Arc;
use tracing::{info, instrument, error};

use crate::{
    app_error::{AppError, AppResult},
    domain::entities::transaction::{Transaction, TransactionStatus},
};

#[derive(Debug, Clone)]
pub struct CheckoutSessionStatus {
    pub payment_status: String,
    pub payment_intent_id: Option<String>,
}

#[async_trait]
pub trait TransactionPersistence: Send + Sync {
    async fn create(&self, transaction: &Transaction) -> AppResult<Transaction>;
    async fn update(&self, transaction: &Transaction) -> AppResult<Transaction>;
    async fn get_by_session_id(&self, session_id: &str) -> AppResult<Transaction>;
}

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    async fn create_checkout_session(
        &self,
        amount: i64,
        currency: &str,
        success_url: &str,
        cancel_url: &str,
        metadata: Option<std::collections::HashMap<String, String>>,
    ) -> AppResult<(String, String)>; // (client_secret, session_id)

    async fn get_checkout_session_status(
        &self,
        session_id: &str,
    ) -> AppResult<CheckoutSessionStatus>;
}

#[derive(Clone)]
pub struct PaymentUseCases {
    transaction_persistence: Arc<dyn TransactionPersistence>,
    payment_gateway: Arc<dyn PaymentGateway>,
}

impl PaymentUseCases {
    pub fn new(
        transaction_persistence: Arc<dyn TransactionPersistence>,
        payment_gateway: Arc<dyn PaymentGateway>,
    ) -> Self {
        Self {
            transaction_persistence,
            payment_gateway,
        }
    }

    #[instrument(skip(self))]
    pub async fn create_checkout_session(
        &self,
        amount: i64, // Amount in cents
        currency: String,
        success_url: String,
        cancel_url: String,
    ) -> AppResult<(String, String)> {
        info!("Initiating checkout session creation...");

        // 1. Create Stripe Session
        let (client_secret, session_id) = self.payment_gateway.create_checkout_session(
            amount,
            &currency,
            &success_url,
            &cancel_url,
            None, // Metadata
        ).await.map_err(|e| {
            error!("Failed to create stripe session: {:?}", e);
            e
        })?;

        // 2. Create Transaction
        let transaction = Transaction::new(session_id.clone(), Some(amount), Some(currency.clone()));

        // 3. Persist Transaction
        self.transaction_persistence.create(&transaction).await.map_err(|e| {
             error!("Failed to persist transaction: {:?}", e);
             e
        })?;

        info!("Checkout session created successfully. Transaction ID: {}", transaction.id);

        Ok((client_secret, session_id))
    }

    #[instrument(skip(self))]
    pub async fn verify_payment_completed(&self, checkout_session_id: &str) -> AppResult<()> {
        info!("Verifying checkout session payment for {}...", checkout_session_id);

        let mut transaction = self
            .transaction_persistence
            .get_by_session_id(checkout_session_id)
            .await
            .map_err(|e| {
                error!("Failed to fetch transaction for checkout session {}: {:?}", checkout_session_id, e);
                e
            })?;

        if transaction.status == TransactionStatus::Completed {
            info!("Transaction already marked completed for {}", checkout_session_id);
            return Ok(());
        }

        let checkout_status = self
            .payment_gateway
            .get_checkout_session_status(checkout_session_id)
            .await
            .map_err(|e| {
                error!("Failed to read checkout session status for {}: {:?}", checkout_session_id, e);
                e
            })?;

        if checkout_status.payment_status.to_lowercase() != "paid" {
            error!("Checkout session {} is not paid yet: {}", checkout_session_id, checkout_status.payment_status);
            return Err(AppError::PaymentNotApproved(format!(
                "The payment for checkout session {} is not approved yet.",
                checkout_session_id
            )));
        }

        transaction.status = TransactionStatus::Completed;
        transaction.updated_at = Utc::now();
        transaction.payment_intent_id = checkout_status.payment_intent_id;

        self.transaction_persistence.update(&transaction).await.map_err(|e| {
            error!("Failed to update transaction status for {}: {:?}", checkout_session_id, e);
            e
        })?;

        info!("Checkout session payment approved and transaction completed for {}", checkout_session_id);

        Ok(())
    }
}
