use std::sync::Arc;

use crate::{
    app_error::{AppError, AppResult},
    entities::user::User,
    infra::config::AppConfig,
    use_cases::user::UserJwtService,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    uuid: String,
    name: String,
    verified: bool,
    exp: usize,
}

pub struct JwtService {
    config: Arc<AppConfig>,
}

impl JwtService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
}

impl UserJwtService for JwtService {
    fn generate_token(&self, user: &User) -> AppResult<String> {
        let token = encode(
            &Header::default(),
            &Claims {
                uuid: user.id.to_string(),
                name: user.username.clone(),
                verified: user.verified.unwrap_or(false),
                exp: (Utc::now() + Duration::minutes(120)).timestamp() as usize,
            },
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
        .map_err(|_| AppError::Internal("JWT Creation Failed".into()))?;

        Ok(token)
    }

    fn validate_token(&self, token: &str) -> AppResult<()> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::Unauthorized("Token expired".into())
            }
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                AppError::Unauthorized("Invalid token".into())
            }
            _ => AppError::Unauthorized("Token validation failed".into()),
        })?;

        Ok(())
    }
}
