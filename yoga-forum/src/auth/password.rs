use anyhow::Error;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Invalid password provided")]
    InvalidPassword,
    #[error("Unknown error occurred")]
    Unknown(#[source] Error),
}

#[derive(Clone)]
pub struct PasswordHandler {
    pepper: String,
}

impl PasswordHandler {
    pub fn new(pepper: String) -> Self {
        // TODO: Would be nice to just create the Argon2 struct and use that,
        // but don't want to pollute everything with lifetimes unecessarily
        Self { pepper }
    }

    pub fn get_hash(&self, password: &str) -> Result<String, PasswordError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new_with_secret(
            self.pepper.as_bytes(),
            Algorithm::default(),
            Version::default(),
            Params::default(),
        )
        .unwrap();

        // TODO: Handle this error cleanly
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| {
                tracing::error!("Argon2 password hashing failed: {err:?}");
            })
            .unwrap()
            .to_string();

        Ok(hash)
    }

    pub fn is_pass_valid(&self, password: &str, hash: &str) -> Result<(), PasswordError> {
        let hash = PasswordHash::new(hash).unwrap();
        let argon2 = Argon2::new_with_secret(
            self.pepper.as_bytes(),
            Algorithm::default(),
            Version::default(),
            Params::default(),
        )
        .unwrap();

        if argon2.verify_password(password.as_bytes(), &hash).is_ok() {
            Ok(())
        } else {
            Err(PasswordError::InvalidPassword)
        }
    }
}
