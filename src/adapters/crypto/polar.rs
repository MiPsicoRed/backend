use async_trait::async_trait;
use polar_rs::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    app_error::{AppError, AppResult},
    infra::config::AppConfig,
    use_cases::user::UserPolarService,
};

pub struct PolarService {
    client: PolarClient,
}

impl PolarService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        match config.release_mode {
            true => Self {
                client: PolarClient::new(&config.polar_access_token),
            },
            false => Self {
                client: PolarClient::sandbox(&config.polar_access_token),
            },
        }
    }

    pub async fn get_all_products(&self) -> AppResult<Vec<Product>> {
        let res = self
            .client
            .list_products(&ListParams {
                page: None,
                limit: None,
            })
            .await
            .map_err(|e: PolarError| AppError::ExternalService(e.to_string()))?;

        Ok(res.items)
    }
}

#[async_trait]
impl UserPolarService for PolarService {
    async fn get_or_create_customer(
        &self,
        user_id: &Uuid,
        email: &str,
        name: &str,
    ) -> AppResult<String> {
        // Try to get existing customer by external_id
        match self
            .client
            .get_customer_by_external_id(&user_id.to_string())
            .await
        {
            Ok(customer) => {
                tracing::info!("Found existing Polar customer: {}", customer.id);
                Ok(customer.id)
            }
            Err(polar_rs::PolarError::Api { status: 404, .. }) => {
                // Customer doesn't exist, create new one
                tracing::info!("Creating new Polar customer for user {}", user_id);

                let request = CreateCustomerRequest {
                    email: email.to_string(),
                    name: Some(name.to_string()),
                    external_id: Some(user_id.to_string()),
                    ..Default::default()
                };

                let customer = self.client.create_customer(request).await.map_err(|e| {
                    AppError::ExternalService(format!("Polar customer creation failed: {}", e))
                })?;

                tracing::info!("Created Polar customer: {}", customer.id);
                Ok(customer.id)
            }
            Err(e) => Err(AppError::ExternalService(format!("Polar API error: {}", e))),
        }
    }

    async fn create_checkout_url(
        &self,
        user_id: &Uuid,
        email: &str,
        name: &str,
        product_id: &str,
        success_url: &str,
    ) -> AppResult<String> {
        // Ensure customer exists before creating checkout
        self.get_or_create_customer(user_id, email, name).await?;

        let request = CreateCheckoutRequest {
            products: vec![product_id.to_string()],
            external_customer_id: Some(user_id.to_string()),
            success_url: Some(success_url.to_string()),
            ..Default::default()
        };

        let checkout = self
            .client
            .create_checkout_session(request)
            .await
            .map_err(|e| {
                AppError::ExternalService(format!("Polar checkout creation failed: {}", e))
            })?;

        Ok(checkout.url)
    }

    async fn has_purchased_product(&self, user_id: &Uuid, product_id: &str) -> AppResult<bool> {
        // Try to get customer state
        match self
            .client
            .get_customer_state_by_external_id(&user_id.to_string())
            .await
        {
            Ok(state) => {
                // Check granted benefits for purchases
                Ok(state.granted_benefits.iter().any(|benefit| {
                    benefit.benefit_id == product_id && benefit.revoked_at.is_none()
                }))
            }
            Err(polar_rs::PolarError::Api { status: 404, .. }) => {
                // Customer doesn't exist, so no purchases
                Ok(false)
            }
            Err(e) => Err(AppError::ExternalService(format!(
                "Polar state fetch failed: {}",
                e
            ))),
        }
    }

    async fn get_portal_url(&self, user_id: &Uuid) -> AppResult<String> {
        let state = self
            .client
            .get_customer_state_by_external_id(&user_id.to_string())
            .await
            .map_err(|e| match e {
                polar_rs::PolarError::Api { status: 404, .. } => AppError::ExternalService(
                    "Customer not found. Make a purchase first.".to_string(),
                ),
                _ => AppError::ExternalService(format!("Polar state fetch failed: {}", e)),
            })?;

        let request = CreateCustomerSessionRequest {
            customer_id: Some(state.customer.id),
            ..Default::default()
        };

        let session = self
            .client
            .create_customer_session(request)
            .await
            .map_err(|e| {
                AppError::ExternalService(format!("Polar session creation failed: {}", e))
            })?;

        Ok(session.customer_portal_url)
    }
}
