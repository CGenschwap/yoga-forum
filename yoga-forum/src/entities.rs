use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewStory {
    pub title: String,
    pub author_id: i64,
    pub url: Option<Url>,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Story {
    pub id: i64,
    pub title: String,
    pub author_id: i64,
    pub created_at: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewComment {
    pub text: String,
    pub author_id: i64,
    pub story_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    pub id: i64,
    pub text: String,
    pub author_id: i64,
    pub created_at: i64,
    pub story_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
}
