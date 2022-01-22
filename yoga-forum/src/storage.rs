use crate::entities::{Comment, NewComment, NewStory, Story};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[cfg(feature = "sqlite")]
mod sqlite;

#[cfg(feature = "sqlite")]
pub use sqlite::SqliteStorage;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn list_users(&self);
    async fn create_user(
        &self,
        username: &str,
        password_hash: &str,
    ) -> std::result::Result<i64, StorageError>;
    async fn get_pass_hash(&self, user_id: i64) -> Result<String, StorageError>;
    async fn new_story(&self, story: &NewStory) -> Result<i64, StorageError>;
    async fn new_comment(&self, comment: &NewComment) -> Result<i64, StorageError>;
    async fn find_user(&self, username: &str) -> Result<i64, StorageError>;

    async fn list_stories(
        &self,
        sort_kind: SortKind,
        token: Option<ListStoryToken>,
    ) -> Result<(Vec<Story>, Option<ListStoryToken>), StorageError>;

    async fn list_comments(
        &self,
        story_id: i64,
        sort_kind: SortKind,
        token: Option<ListStoryToken>,
    ) -> Result<(Vec<Comment>, Option<ListStoryToken>), StorageError>;

    async fn get_story(&self, story_id: i64) -> Result<Story, StorageError>;
    async fn get_comment(&self, comment_id: i64) -> Result<Comment, StorageError>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListStoryToken {
    sort_kind: SortKind,
    value: i64,
}

impl ListStoryToken {
    pub fn as_string(&self) -> String {
        let s = serde_json::to_string(&self).unwrap();
        base64::encode(s)
    }

    pub fn from_string(s: &str) -> anyhow::Result<Self> {
        let bytes = base64::decode(s)?;
        let s: String = String::from_utf8(bytes)?;
        let token = serde_json::from_str(&s)?;
        Ok(token)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum SortKind {
    New,
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Username already used")]
    UsernameAlreadyExists(sqlx::Error),

    #[error("User with given username not found.")]
    UserNotFound(sqlx::Error),

    #[error("Password row corresponding to user_id not found")]
    PasswordNotFound(sqlx::Error),

    #[error("The story_id or parent_id provided does not exist")]
    InvalidStoryOrParentId(sqlx::Error),

    #[error("Invalid parent_id for given story_id")]
    InvalidParentIdForStory,

    #[error(
        "Invalid sort token provided. Either it has expired, or it is for the wrong sort type."
    )]
    InvalidSortToken,

    #[error("Unknown error")]
    Unknown(#[from] sqlx::Error),
}

pub type DynStorage = Box<dyn Storage>;

pub struct StorageObj {
    inner: DynStorage,
}

impl StorageObj {
    pub async fn new(data_path: &Path) -> Result<Self> {
        let s = if cfg!(feature = "sqlite") {
            let s = SqliteStorage::new(data_path).await?;
            let s: DynStorage = Box::new(s);
            s
        } else {
            panic!("No other storage implementation implemented");
        };

        Ok(Self { inner: s })
    }

    pub fn as_inner(&self) -> &dyn Storage {
        self.inner.as_ref()
    }
}
