use sqlx::PgPool;

pub mod email;
pub mod patient;
pub mod session_type;
pub mod user;
pub mod user_token;

#[derive(Clone)]
pub struct PostgresPersistence {
    pool: PgPool,
}

impl PostgresPersistence {
    pub fn new(pool: PgPool) -> Self {
        PostgresPersistence { pool }
    }
}
