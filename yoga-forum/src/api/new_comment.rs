use crate::auth::AuthUser;
use crate::constants::TEXT_CHAR_MAX;
use crate::entities::NewComment;
use crate::error::YogaError;
use crate::storage::{Storage, StorageError, StorageObj};
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct NewCommentRequestBody {
    pub text: String,
    pub parent_id: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct NewCommentRequestPath {
    pub story_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct NewCommentResponse {
    pub comment_id: i64,
}

#[tracing::instrument(skip(storage))]
pub async fn new_comment(
    storage: web::Data<StorageObj>,
    auth: AuthUser,
    req_body: web::Json<NewCommentRequestBody>,
    req_path: web::Path<NewCommentRequestPath>,
) -> Result<HttpResponse, Error> {
    let res = new_comment_inner(
        storage.as_inner(),
        auth.user_id,
        req_path.story_id,
        req_body.parent_id,
        &req_body.text,
    )
    .await?;
    Ok(HttpResponse::Ok().json(&res))
}

async fn new_comment_inner(
    storage: &dyn Storage,
    user_id: i64,
    story_id: i64,
    parent_id: Option<i64>,
    text: &str,
) -> Result<NewCommentResponse, YogaError> {
    if text.len() > TEXT_CHAR_MAX {
        return Err(YogaError::TextTooLong("text", TEXT_CHAR_MAX));
    }

    let comment = NewComment {
        text: text.to_string(),
        author_id: user_id,
        story_id,
        parent_id,
    };
    let id = storage
        .new_comment(&comment)
        .await
        .map_err(|err| match err {
            StorageError::InvalidStoryOrParentId(_) => {
                YogaError::InvalidStoryOrParentId(err.into())
            }
            _ => YogaError::Unknown(err.into()),
        })?;
    Ok(NewCommentResponse { comment_id: id })
}
