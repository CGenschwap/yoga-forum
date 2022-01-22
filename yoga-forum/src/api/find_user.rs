use crate::error::YogaError;
use crate::storage::{Storage, StorageError, StorageObj};
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct FindUserResponse {
    pub user_id: i64,
}

#[tracing::instrument(skip(storage))]
pub async fn find_user(
    storage: web::Data<StorageObj>,
    username: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let res = find_user_inner(storage.as_inner(), &username).await?;
    Ok(HttpResponse::Ok().json(&res))
}

async fn find_user_inner(
    storage: &dyn Storage,
    username: &str,
) -> Result<FindUserResponse, YogaError> {
    let id = storage.find_user(username).await.map_err(|err| match err {
        StorageError::UserNotFound(_) => YogaError::UserNotFound(err.into()),
        _ => YogaError::Unknown(err.into()),
    })?;
    Ok(FindUserResponse { user_id: id })
}
