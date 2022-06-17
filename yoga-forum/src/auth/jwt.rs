use anyhow::Result;
use chrono::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct JwtHandler {
    secret: String,
}

impl JwtHandler {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn create(&self, user_id: i64) -> Result<String> {
        let iat = Utc::now();
        let exp = iat + chrono::Duration::days(1);
        let iat = iat.timestamp();
        let nbf = iat;
        let exp = exp.timestamp();

        let claims = Claims {
            sub: user_id,
            iat,
            nbf,
            exp,
        };
        let jwt = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;
        Ok(jwt)
    }

    pub fn parse(&self, token: String) -> Result<Claims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 30;
        validation.validate_exp = true;
        validation.validate_nbf = true;

        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &validation
        )?;
        Ok(token.claims)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub iat: i64,
    pub exp: i64,
    pub nbf: i64,
}
