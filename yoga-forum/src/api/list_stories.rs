use crate::entities::Story;
use crate::error::YogaError;
use crate::storage::{ListStoryToken, SortKind, Storage, StorageObj};
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ListStoriesRequest {
    next_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ListStoriesResponse {
    pub stories: Vec<Story>,
    pub next_token: Option<String>,
}

#[tracing::instrument(skip(storage))]
pub async fn list_stories(
    storage: web::Data<StorageObj>,
    req: web::Query<ListStoriesRequest>,
) -> Result<HttpResponse, Error> {
    let res = list_stories_inner(storage.as_inner(), &req).await?;
    Ok(HttpResponse::Ok().json(&res))
}

async fn list_stories_inner(
    storage: &dyn Storage,
    req: &ListStoriesRequest,
) -> Result<ListStoriesResponse, YogaError> {
    let token = req
        .next_token
        .as_ref()
        .map(|t| ListStoryToken::from_string(t))
        .transpose()?;

    let (stories, token) = storage
        .list_stories(SortKind::New, token)
        .await
        .map_err(|err| YogaError::Unknown(err.into()))?;

    // TODO: This should be a configurable value; it needs
    // to be kept in sync with our DB implemenation
    let next_token = if stories.len() < 25 {
        None
    } else {
        token.map(|t| t.as_string())
    };

    Ok(ListStoriesResponse {
        stories,
        next_token,
    })
}
