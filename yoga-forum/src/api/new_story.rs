use crate::auth::AuthUser;
use crate::constants::{TEXT_CHAR_MAX, TITLE_CHAR_MAX};
use crate::entities::NewStory;
use crate::error::YogaError;
use crate::storage::{Storage, StorageObj};
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Debug)]
pub struct NewStoryRequest {
    pub title: String,
    pub url: Option<Url>,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewStoryResponse {
    pub story_id: i64,
}

#[tracing::instrument(skip(storage))]
pub async fn new_story(
    storage: web::Data<StorageObj>,
    auth: AuthUser,
    req: web::Json<NewStoryRequest>,
) -> Result<HttpResponse, Error> {
    let res = new_story_inner(storage.as_inner(), auth.user_id, &req).await?;
    Ok(HttpResponse::Ok().json(&res))
}

async fn new_story_inner(
    storage: &dyn Storage,
    user_id: i64,
    new_story: &NewStoryRequest,
) -> Result<NewStoryResponse, YogaError> {
    if let Some(text) = &new_story.text {
        if text.len() > TEXT_CHAR_MAX {
            return Err(YogaError::TextTooLong("text", TEXT_CHAR_MAX));
        }
    }

    if new_story.title.len() > TITLE_CHAR_MAX {
        return Err(YogaError::TextTooLong("title", TITLE_CHAR_MAX));
    }

    // TODO: Is this the right length for a URL?
    if let Some(url) = &new_story.url {
        if url.as_str().len() > TEXT_CHAR_MAX {
            return Err(YogaError::TextTooLong("url", TEXT_CHAR_MAX));
        }
    }

    // TODO: Use a NewStory type
    let story = NewStory {
        title: new_story.title.clone(),
        // TODO: Must be a cleaner way of doing this
        url: new_story.url.clone(),
        text: new_story.text.clone(),
        author_id: user_id,
    };
    let id = storage
        .new_story(&story)
        .await
        .map_err(|err| YogaError::Unknown(err.into()))?;
    Ok(NewStoryResponse { story_id: id })
}
