use async_trait::async_trait;

use std::sync::Arc;
use tracing::{info, instrument, error};

use crate::{
    app_error::AppResult,
    domain::entities::transaction::Transaction,
};

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
    ) -> AppResult<String> {
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

        Ok(client_secret)
    }
}
