use std::sync::Arc;

use stripe::CheckoutSession;
use stripe::Client;
use async_trait::async_trait;
use tracing::error;

use crate::{
    app_error::{AppError, AppResult},
    application::use_cases::payment::PaymentGateway,
    infra::config::AppConfig,
};

#[derive(Clone)]
pub struct StripeGateway {
    client: Client,
}

impl StripeGateway {
    pub fn new(config: Arc<AppConfig>) -> Self {
        let client = Client::new(config.stripe_secret_key.clone());
        Self { client }
    }
}

#[async_trait]
impl PaymentGateway for StripeGateway {
    async fn create_checkout_session(
        &self,
        amount: i64,
        currency: &str,
        success_url: &str,
        cancel_url: &str,
        metadata: Option<std::collections::HashMap<String, String>>,
    ) -> AppResult<(String, String)> {
        use stripe::{
            CheckoutSessionMode, CreateCheckoutSession, CreateCheckoutSessionLineItems,
            CreateCheckoutSessionLineItemsPriceData,
            CreateCheckoutSessionLineItemsPriceDataProductData,
        };

        let mut create_session = CreateCheckoutSession::new();
        create_session.mode = Some(CheckoutSessionMode::Payment);
        create_session.ui_mode = Some(stripe::CheckoutSessionUiMode::Embedded);
        create_session.return_url = Some(success_url);
        
        let price_data = CreateCheckoutSessionLineItemsPriceData {
            currency: currency.parse().unwrap_or(stripe::Currency::EUR), // Default or error handle
            product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                name: "Session Booking".to_string(), // Could be dynamic
                ..Default::default()
            }),
            unit_amount: Some(amount), // Stripe expects amount in cents
            ..Default::default()
        };

        let line_item = CreateCheckoutSessionLineItems {
            quantity: Some(1),
            price_data: Some(price_data),
            ..Default::default()
        };

        create_session.line_items = Some(vec![line_item]);

        if let Some(meta) = metadata {
             // TODO: [lazaropaul] In "final" app, we would have to map `metadata`
             // (Transaction info) to Stripe metadata create_session.metadata = ...
        }

        let session = CheckoutSession::create(&self.client, create_session)
            .await
            .map_err(|e| {
                error!("Stripe create session error: {:?}", e);
                AppError::ExternalServiceError(format!("Stripe error: {}", e))
            })?;

        let client_secret = session.client_secret.ok_or_else(|| {
             error!("Stripe did not return a client_secret");
             AppError::ExternalServiceError("Stripe did not return a client_secret".into())
        })?;

        let id = session.id.as_str().to_string();

        Ok((client_secret, id))
    }
}

