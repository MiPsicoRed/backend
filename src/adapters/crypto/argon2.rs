use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use crate::{
    app_error::{AppError, AppResult},
    use_cases::user::UserCredentialsHasher,
};

#[derive(Default)]
pub struct ArgonPasswordHasher {
    hasher: Argon2<'static>,
}

impl UserCredentialsHasher for ArgonPasswordHasher {
    fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self
            .hasher
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::Internal("Password hashing failed.".into()))?
            .to_string();

        Ok(hash)
    }

    fn verify_password(&self, user_password_hash: &str, input_password: &str) -> AppResult<()> {
        let parsed_hash = PasswordHash::new(user_password_hash)
            .map_err(|_| AppError::Internal("Invalid password hash format.".into()))?;

        self.hasher
            .verify_password(input_password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::InvalidCredentials)
    }
}
