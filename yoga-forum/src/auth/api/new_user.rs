use crate::auth::PasswordHandler;
use crate::constants::{PASSWORD_CHAR_MAX, USERNAME_CHAR_MAX};
use crate::error::YogaError;
use crate::storage::{Storage, StorageError, StorageObj};
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct NewUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewUserResponse {
    pub user_id: i64,
}

#[tracing::instrument(skip(storage, password_handler))]
pub async fn new_user(
    storage: web::Data<StorageObj>,
    password_handler: web::Data<PasswordHandler>,
    body: web::Json<NewUserRequest>,
) -> Result<HttpResponse, Error> {
    let res = new_user_inner(storage.as_inner(), &password_handler, &body).await?;
    Ok(HttpResponse::Ok().json(&res))
}

async fn new_user_inner(
    storage: &dyn Storage,
    password_handler: &PasswordHandler,
    new_user: &NewUserRequest,
) -> Result<NewUserResponse, YogaError> {
    if new_user.username.len() > USERNAME_CHAR_MAX {
        return Err(YogaError::TextTooLong("username", USERNAME_CHAR_MAX));
    }

    // TODO: Is there a better max length for passwords?
    // It doesn't _really_ matter b/c we hash
    if new_user.password.len() > PASSWORD_CHAR_MAX {
        return Err(YogaError::TextTooLong("password", PASSWORD_CHAR_MAX));
    }

    // TODO: It would be ideal if instead of validation we'd parse. Or
    // maybe have this as an Actix-web extractor to avoid future errors.
    // Probably overkill for right now
    check_valid(&new_user.username)?;

    let pass_hash = password_handler
        .get_hash(&new_user.password)
        .map_err(|err| YogaError::Unknown(err.into()))?;
    let user_id = storage
        .create_user(&new_user.username, pass_hash.as_str())
        .await
        .map_err(|err| match err {
            StorageError::UsernameAlreadyExists(_) => YogaError::UsernameAlreadyExists(err.into()),
            _ => YogaError::Unknown(err.into()),
        })?;
    Ok(NewUserResponse { user_id })
}

fn check_valid(username: &str) -> Result<(), YogaError> {
    let r = username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');

    if r {
        Ok(())
    } else {
        Err(YogaError::InvalidUsername)
    }
}
