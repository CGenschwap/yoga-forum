use crate::entities::Comment;
use crate::error::YogaError;
use crate::storage::{ListStoryToken, SortKind, Storage, StorageObj};
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Deserialize, Debug)]
pub struct ListCommentsRequestQuery {
    next_token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ListCommentsRequestPath {
    story_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListCommentsResponse {
    pub comments: Vec<Comment>,
    pub next_token: Option<String>,
}

#[tracing::instrument(skip(storage))]
pub async fn list_comments(
    storage: web::Data<StorageObj>,
    path: web::Path<ListCommentsRequestPath>,
    query: web::Query<ListCommentsRequestQuery>,
) -> Result<HttpResponse, Error> {
    let res = list_comments_inner(
        storage.as_inner(),
        path.story_id,
        query.next_token.as_deref(),
    )
    .await?;
    debug!("{res:?}");
    Ok(HttpResponse::Ok().json(&res))
}

async fn list_comments_inner(
    storage: &dyn Storage,
    story_id: i64,
    next_token: Option<&str>,
) -> Result<ListCommentsResponse, YogaError> {
    let token = next_token.map(ListStoryToken::from_string).transpose()?;

    let (comments, token) = storage
        .list_comments(story_id, SortKind::New, token)
        .await
        .map_err(|err| YogaError::Unknown(err.into()))?;

    // TODO: This should be a configurable value; it needs
    // to be kept in sync with our DB implemenation
    let next_token = if comments.len() < 50 {
        None
    } else {
        token.map(|t| t.as_string())
    };

    Ok(ListCommentsResponse {
        comments,
        next_token,
    })
}
