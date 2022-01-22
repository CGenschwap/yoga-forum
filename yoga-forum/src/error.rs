//! Web-Level Error Handling
//!
//! This module contains all relevant code to web-level error handling. Specifically the errors
//! that the _user_ will see. Right now it is done in a fairly hacky way as a temporary solution to
//! get things moving.
//!
//! TODO: There is  likely a way to make all of this error-handling much nicer and less
//! boilerplate-y. Avoiding Macros for now, but probably a non-macro solution as well
//!
use actix_web::{error::ResponseError, HttpResponse};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use thiserror::Error;

/// Errors served to the end-user
///
/// This Enum contains errors that will eventually surface to the end-user.
/// The Enum should _not_ be used in internal modules, and should only be used by the API
/// interface.
#[derive(Error, Debug)]
pub enum YogaError {
    #[error("Username already exists. Please choose a new username or sign in via the /api/v1/login endpoint.")]
    UsernameAlreadyExists(#[source] Error),

    #[error("Invalid username or password. Please double-check your username or password and try again.")]
    InvalidUsernameOrPass(#[source] Error),

    #[error("Invalid Bearer token. {0}")]
    InvalidBearerToken(#[source] Error),

    #[error("User not found for the requested username.")]
    UserNotFound(#[source] Error),

    #[error("Invalid media type for the request. Ensure the media type is \"Content-Type: application/json\". Further details: {0}")]
    InvalidMediaType(#[source] Error),

    #[error("Request is invalid and either does not contain a valid field, or is missing a required field. Please double check your request. Further details: {0}")]
    InvalidRequest(#[source] Error),

    #[error("The story_id or parent_id provided could not be found. Please double-check that the provided IDs are correct")]
    InvalidStoryOrParentId(#[source] Error),

    #[error("{0} exceeds the {1} character limit.")]
    TextTooLong(&'static str, usize),

    #[error(
        "Request was rate-limited. Please try to limit API calls to prevent us from going down!"
    )]
    RateLimited,

    #[error("Username contains characters that are not allowed. Usernames must be ASCII alpha-numeric or '-', '.' and '_', regex [a-z0-9-_]")]
    InvalidUsername,

    #[error("Unknown internal server error occurred. We're a small team but we're working to make sure there are 0 errors like this. Please try again and feel free to file a bug with recreation details if it keeps occurring!")]
    Unknown(#[source] Error),
}

impl ResponseError for YogaError {
    fn error_response(&self) -> HttpResponse {
        tracing::error!("{self} => {:?}", self.source());

        let message = self.to_json();

        use YogaError::*;
        match self {
            UsernameAlreadyExists(_) => HttpResponse::Conflict().json(message),
            InvalidUsernameOrPass(_) => HttpResponse::Unauthorized().json(message),
            InvalidBearerToken(_) => HttpResponse::Unauthorized().json(message),
            UserNotFound(_) => HttpResponse::NotFound().json(message),
            InvalidMediaType(_) => HttpResponse::UnsupportedMediaType().json(message),
            InvalidStoryOrParentId(_) => HttpResponse::NotFound().json(message),
            InvalidRequest(_) => HttpResponse::BadRequest().json(message),
            RateLimited => HttpResponse::TooManyRequests().json(message),
            TextTooLong(_, _) => HttpResponse::BadRequest().json(message),
            InvalidUsername => HttpResponse::BadRequest().json(message),
            Unknown(_) => HttpResponse::InternalServerError().json(message),
        }
    }
}

impl YogaError {
    pub fn to_code(&self) -> &'static str {
        use YogaError::*;
        match self {
            UsernameAlreadyExists(_) => "USERNAME_EXISTS",
            InvalidUsernameOrPass(_) => "INVALID_USERNAME_OR_PASS",
            InvalidBearerToken(_) => "INVALID_BEARER",
            UserNotFound(_) => "USER_NOT_FOUND",
            InvalidMediaType(_) => "INVALID_MEDIA_TYPE",
            InvalidRequest(_) => "INVALID_REQUEST",
            InvalidStoryOrParentId(_) => "INVALID_STORY_OR_PARENT_ID",
            RateLimited => "RATE_LIMITED",
            TextTooLong(_, _) => "TEXT_TOO_LONG",
            InvalidUsername => "INVALID_USERNAME",
            Unknown(_) => "UKNOWN",
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        // TODO: There is likely a cleaner way of doing this
        serde_json::to_value(ErrorResponse {
            error_code: self.to_code().to_owned(),
            error_message: format!("{self}"),
        })
        .unwrap()
    }
}

impl From<anyhow::Error> for YogaError {
    fn from(err: anyhow::Error) -> Self {
        Self::Unknown(err)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error_code: String,
    pub error_message: String,
}
