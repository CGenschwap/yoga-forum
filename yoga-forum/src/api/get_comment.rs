use crate::entities::Comment;
use crate::error::YogaError;
use crate::storage::{Storage, StorageObj};
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct GetCommentRequest {
    comment_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetCommentResponse {
    pub comment: Comment,
}

#[tracing::instrument(skip(storage))]
pub async fn get_comment(
    storage: web::Data<StorageObj>,
    req: web::Path<GetCommentRequest>,
) -> Result<HttpResponse, Error> {
    let res = get_comment_inner(storage.as_inner(), req.comment_id).await?;
    Ok(HttpResponse::Ok().json(&res))
}

async fn get_comment_inner(
    storage: &dyn Storage,
    comment_id: i64,
) -> Result<GetCommentResponse, YogaError> {
    let comment = storage
        .get_comment(comment_id)
        .await
        .map_err(|err| YogaError::Unknown(err.into()))?;

    Ok(GetCommentResponse { comment })
}
