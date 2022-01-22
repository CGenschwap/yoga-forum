use super::jwt::JwtHandler;
use crate::error::YogaError;
use actix_web::{dev::Payload, http::header, web, Error, HttpRequest};
use anyhow::anyhow;
use std::future::{ready, Ready};
use thiserror::Error;

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: i64,
}

impl actix_web::FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    #[tracing::instrument(skip(_payload))]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        tracing::info!("HERE");
        let jwt_handler = req.app_data::<web::Data<JwtHandler>>();
        let jwt_handler = if let Some(jwt_handler) = jwt_handler {
            jwt_handler
        } else {
            // TODO: Specific error here
            tracing::error!("No jwt_handler. This should never happen");
            return ready(Err(YogaError::Unknown(anyhow!("No JWT handler")).into()));
        };

        let token = match extract_bearer_token(req.headers()) {
            Ok(b) => b,
            Err(err) => {
                tracing::error!("Unable to extract Bearer token. {err:?}");
                return ready(Err(YogaError::InvalidBearerToken(err.into()).into()));
            }
        };

        tracing::debug!("{token:?}");

        let parse = jwt_handler.parse(token);
        tracing::debug!("{parse:?}");

        let claims = match parse {
            Ok(claims) => claims,
            Err(err) => {
                tracing::error!("Invalid token claims. {err:?}");
                return ready(Err(YogaError::InvalidBearerToken(err).into()));
            }
        };
        tracing::debug!("{claims:?}");

        ready(Ok(Self {
            user_id: claims.sub,
        }))
    }
}

fn extract_bearer_token(header_map: &header::HeaderMap) -> Result<String, BearerExtractError> {
    // "Authorization: {str}"
    let authorization_value = header_map
        .get(header::AUTHORIZATION)
        .ok_or(BearerExtractError::NoAuthorizationHeader)?
        .to_str()
        .ok()
        .ok_or(BearerExtractError::InvalidBearerEncoding)?;

    // "Authorization: Bearer {?}"
    let mut parts = authorization_value.splitn(2, ' ');
    let token = if parts.next() == Some("Bearer") {
        parts.next()
    } else {
        return Err(BearerExtractError::InvalidBearerMissingScheme);
    };

    // "Authorization: Bearer {str}"
    let token = token.ok_or(BearerExtractError::InvalidBearerMissingToken)?;
    Ok(token.to_string())
}

// TODO: Make these errors nice and helpful
#[derive(Error, Debug)]
enum BearerExtractError {
    #[error("No authorization header provided. Needs to have a header set as `Authorization: Bearer {{token}}`")]
    NoAuthorizationHeader,
    #[error("Bearer encoding incorrect. Format should be `Authorization: Bearer {{token}}` with valid base64 encoding.")]
    InvalidBearerEncoding,
    #[error("Bearer scheme is missing. Format should be `Authorization: Bearer {{token}}`")]
    InvalidBearerMissingScheme,
    #[error("No auth token provided. Format should be `Authorization: Bearer {{token}}`")]
    InvalidBearerMissingToken,
}
