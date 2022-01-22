use crate::auth::{JwtHandler, PasswordError, PasswordHandler};
use crate::error::YogaError;
use crate::storage::{Storage, StorageError, StorageObj};
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: Option<String>,
    pub user_id: Option<i64>,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[tracing::instrument(skip(storage, jwt_handler, password_handler, req))]
pub async fn login(
    storage: web::Data<StorageObj>,
    password_handler: web::Data<PasswordHandler>,
    jwt_handler: web::Data<JwtHandler>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, Error> {
    let res = login_inner(storage.as_inner(), &password_handler, &jwt_handler, &req).await?;
    Ok(HttpResponse::Ok().json(&res))
}

async fn login_inner(
    storage: &dyn Storage,
    password_handler: &PasswordHandler,
    jwt_handler: &JwtHandler,
    req: &LoginRequest,
) -> Result<LoginResponse, YogaError> {
    let user_id = if let Some(user_id) = req.user_id {
        user_id
    } else if let Some(username) = &req.username {
        storage.find_user(username).await.map_err(|err| match err {
            StorageError::UserNotFound(_) => YogaError::UserNotFound(err.into()),
            _ => YogaError::Unknown(err.into()),
        })?
    } else {
        return Err(YogaError::InvalidRequest(anyhow::anyhow!(
            "user_id or username not provided"
        )));
    };

    let hash = storage
        .get_pass_hash(user_id)
        .await
        .map_err(|err| match err {
            StorageError::PasswordNotFound(_) => YogaError::InvalidUsernameOrPass(err.into()),
            _ => YogaError::Unknown(err.into()),
        })?;
    let _ = password_handler
        .is_pass_valid(&req.password, &hash)
        .map_err(|err| match err {
            PasswordError::InvalidPassword => YogaError::InvalidUsernameOrPass(err.into()),
            _ => YogaError::Unknown(err.into()),
        })?;

    let token = jwt_handler.create(user_id)?;
    Ok(LoginResponse { token })
}
