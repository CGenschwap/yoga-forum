use crate::entities::Story;
use crate::error::YogaError;
use crate::storage::{Storage, StorageObj};
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct GetStoryRequest {
    story_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetStoryResponse {
    pub story: Story,
}

#[tracing::instrument(skip(storage))]
pub async fn get_story(
    storage: web::Data<StorageObj>,
    req: web::Path<GetStoryRequest>,
) -> Result<HttpResponse, Error> {
    let res = get_story_inner(storage.as_inner(), req.story_id).await?;
    Ok(HttpResponse::Ok().json(&res))
}

async fn get_story_inner(
    storage: &dyn Storage,
    story_id: i64,
) -> Result<GetStoryResponse, YogaError> {
    let story = storage
        .get_story(story_id)
        .await
        .map_err(|err| YogaError::Unknown(err.into()))?;

    Ok(GetStoryResponse { story })
}
