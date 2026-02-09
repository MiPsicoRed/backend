use std::sync::Arc;

use axum::{extract::{State, Json}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{
    app_error::AppResult,
    application::use_cases::payment::PaymentUseCases,
};

#[derive(Deserialize)]
pub struct CreateCheckoutSessionRequest {
    pub amount: i64,
    pub currency: String,
    pub success_url: String,
    pub cancel_url: String,
}

#[derive(Serialize)]
pub struct CreateCheckoutSessionResponse {
    pub client_secret: String,
}

pub async fn create_checkout_session(
    State(payment_use_cases): State<Arc<PaymentUseCases>>,
    Json(payload): Json<CreateCheckoutSessionRequest>,
) -> AppResult<impl IntoResponse> {
    let client_secret = payment_use_cases.create_checkout_session(
        payload.amount,
        payload.currency,
        payload.success_url,
        payload.cancel_url,
    ).await?;

    Ok((StatusCode::OK, Json(CreateCheckoutSessionResponse { client_secret })))
}
