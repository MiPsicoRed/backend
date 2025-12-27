use std::sync::Arc;

use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use polar_rs::PolarClient;

use crate::{
    app_error::{AppError, AppResult},
    infra::config::AppConfig,
    use_cases::user::{UserCredentialsHasher, UserPolarService},
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
}

impl UserPolarService for PolarService {}
