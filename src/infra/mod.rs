use std::sync::Arc;

use crate::{
    adapters::{
        crypto::{argon2::ArgonPasswordHasher, jwt::JwtService},
        persistence::PostgresPersistence,
    },
    infra::{config::AppConfig, db::init_db},
};

pub mod app;
pub mod config;
pub mod db;
pub mod setup;

pub async fn postgres_persistence() -> anyhow::Result<PostgresPersistence> {
    let pool = init_db().await?;
    let persistence = PostgresPersistence::new(pool);
    Ok(persistence)
}

pub fn argon2_password_hasher() -> ArgonPasswordHasher {
    ArgonPasswordHasher::default()
}

pub fn jwt_service(config: Arc<AppConfig>) -> JwtService {
    JwtService::new(config)
}
