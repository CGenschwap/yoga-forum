use super::{ListStoryToken, SortKind, Storage, StorageError};
use crate::entities::{Comment, NewComment, NewStory, Story};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{sqlite::SqliteRow, Acquire, Row, SqlitePool};
use std::path::Path;
use url::Url;

#[derive(Debug, Clone)]
pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    pub async fn new(data_path: &Path) -> Result<Self> {
        let db_path = data_path.join("db.sqlite3");
        if !db_path.exists() {
            std::fs::File::create(&db_path)?;
        }
        let pool = SqlitePool::connect(db_path.to_str().unwrap()).await?;

        let _res = sqlx::migrate!("src/storage/migrations-sqlite")
            .run(&pool)
            .await?;

        // TODO: Is this needed?
        let _res = sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    #[tracing::instrument(skip(self))]
    async fn list_users(&self) {
        let _users = sqlx::query("").fetch_all(&self.pool).await.unwrap();
    }

    #[tracing::instrument(skip(self))]
    async fn new_story(&self, story: &NewStory) -> Result<i64, StorageError> {
        let mut conn = self.pool.acquire().await?;
        let res = sqlx::query(
            r#"
            INSERT INTO stories (title, url, text, author_id)
            VALUES (?1, ?2, ?3, ?4)
            "#,
        )
        .bind(&story.title)
        // TODO: Probably a better way of doing this to avoid the clone
        .bind(&story.url.clone().map(|url| url.to_string()))
        .bind(&story.text)
        .bind(story.author_id)
        .execute(&mut conn)
        .await?
        .last_insert_rowid();

        Ok(res)
    }

    #[tracing::instrument()]
    async fn create_user(&self, username: &str, password_hash: &str) -> Result<i64, StorageError> {
        let mut conn = self.pool.acquire().await?;
        let mut transaction = conn.begin().await?;

        let normalized_username = username.to_ascii_lowercase();

        let id = sqlx::query(
            r#"
            INSERT INTO users (username, normalized_username)
            VALUES (?1, ?2);
            "#,
        )
        .bind(&username)
        .bind(&normalized_username)
        .execute(&mut transaction)
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(ref db_err) => {
                if let Some("2067") = db_err.code().as_deref() {
                    StorageError::UsernameAlreadyExists(err)
                } else {
                    StorageError::Unknown(err)
                }
            }
            _ => StorageError::Unknown(err),
        })?
        .last_insert_rowid();

        sqlx::query(
            r#"
            INSERT INTO passwords (user_id, hash)
            VALUES (?1, ?2);
            "#,
        )
        .bind(id)
        .bind(&password_hash)
        .execute(&mut transaction)
        .await?;

        transaction.commit().await?;
        Ok(id)
    }

    #[tracing::instrument(skip(self))]
    async fn new_comment(&self, comment: &NewComment) -> Result<i64, StorageError> {
        let mut conn = self.pool.acquire().await?;

        // TODO: We check that the parent comment is a part of the same
        // story, but we might be able to have this as a database check-constraint
        if let Some(parent_id) = comment.parent_id {
            let parent = self.get_comment(parent_id).await?;
            if parent.story_id != comment.story_id {
                return Err(StorageError::InvalidParentIdForStory);
            }
        }

        let res = sqlx::query(
            r#"
            INSERT INTO comments (
                text
                , author_id
                , story_id
                , parent_id
            )
            VALUES (?1, ?2, ?3, ?4)
            "#,
        )
        .bind(&comment.text)
        .bind(comment.author_id)
        .bind(comment.story_id)
        .bind(comment.parent_id)
        .execute(&mut conn)
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(ref db_err) => {
                if let Some("787") = db_err.code().as_deref() {
                    StorageError::InvalidStoryOrParentId(err)
                } else {
                    StorageError::Unknown(err)
                }
            }
            _ => StorageError::Unknown(err),
        })?
        .last_insert_rowid();

        Ok(res)
    }

    #[tracing::instrument(skip(self))]
    async fn get_pass_hash(&self, user_id: i64) -> Result<String, StorageError> {
        let mut conn = self.pool.acquire().await?;

        let rec = sqlx::query(
            r#"
           SELECT hash
           FROM passwords
           WHERE user_id = ?1;
           "#,
        )
        .bind(user_id)
        .fetch_one(&mut conn)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => StorageError::PasswordNotFound(err),
            _ => StorageError::Unknown(err),
        })?;
        Ok(rec.get("hash"))
    }

    #[tracing::instrument(skip(self))]
    async fn find_user(&self, username: &str) -> Result<i64, StorageError> {
        let mut conn = self.pool.acquire().await?;

        let rec = sqlx::query(
            r#"
            SELECT id
            FROM users
            WHERE username = ?1;
            "#,
        )
        .bind(username)
        .fetch_one(&mut conn)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => StorageError::UserNotFound(err),
            _ => StorageError::Unknown(err),
        })?;
        Ok(rec.get("id"))
    }

    #[tracing::instrument(skip(self))]
    async fn list_stories(
        &self,
        sort_kind: SortKind,
        token: Option<ListStoryToken>,
    ) -> Result<(Vec<Story>, Option<ListStoryToken>), StorageError> {
        if let Some(ref token) = token {
            if token.sort_kind != sort_kind {
                return Err(StorageError::InvalidSortToken);
            }
        }

        let mut conn = self.pool.acquire().await?;

        let query = match sort_kind {
            SortKind::New => LIST_STORIES_NEW,
        };

        // TODO: Add token filtering
        let rec = sqlx::query(query)
            .bind(token.map(|t| t.value))
            .map(|row: SqliteRow| {
                // TODO: Might want to error-handle here
                let url: Option<String> = row.get(3);
                let url = url.map(|u| Url::parse(&u).unwrap());
                Story {
                    id: row.get(0),
                    title: row.get(1),
                    author_id: row.get(2),
                    url,
                    text: row.get(4),
                    created_at: row.get(5),
                }
            })
            .fetch_all(&mut conn)
            .await?;

        let token = if !rec.is_empty() {
            let token_value = rec.iter().last().unwrap().created_at.to_owned();

            Some(ListStoryToken {
                sort_kind: SortKind::New,
                value: token_value,
            })
        } else {
            None
        };

        Ok((rec, token))
    }

    #[tracing::instrument(skip(self))]
    async fn get_story(&self, story_id: i64) -> Result<Story, StorageError> {
        let mut conn = self.pool.acquire().await?;

        // TODO: Add token filtering
        // TODO: These SQL queries can be combined
        let story = sqlx::query(
            r#"
            SELECT id
                , title
                , author_id
                , url
                , text
                , created_at
            FROM stories
            WHERE stories.id = ?1
            "#,
        )
        .bind(story_id)
        .map(|row: SqliteRow| {
            // TODO: Might want to error-handle here
            let url: Option<String> = row.get(3);
            let url = url.map(|u| Url::parse(&u).unwrap());
            Story {
                id: row.get(0),
                title: row.get(1),
                author_id: row.get(2),
                url,
                text: row.get(4),
                created_at: row.get(5),
            }
        })
        .fetch_one(&mut conn)
        .await?;

        Ok(story)
    }

    #[tracing::instrument(skip(self))]
    async fn list_comments(
        &self,
        story_id: i64,
        sort_kind: SortKind,
        token: Option<ListStoryToken>,
    ) -> Result<(Vec<Comment>, Option<ListStoryToken>), StorageError> {
        if let Some(ref token) = token {
            if token.sort_kind != sort_kind {
                return Err(StorageError::InvalidSortToken);
            }
        }

        let mut conn = self.pool.acquire().await?;

        let query = match sort_kind {
            SortKind::New => LIST_COMMENTS_NEW,
        };

        let rec = sqlx::query(query)
            .bind(story_id)
            .bind(token.map(|t| t.value))
            .map(|row: SqliteRow| Comment {
                id: row.get(0),
                author_id: row.get(1),
                text: row.get(2),
                parent_id: row.get(3),
                created_at: row.get(4),
                story_id,
            })
            .fetch_all(&mut conn)
            .await?;

        let token = if !rec.is_empty() {
            let token_value = rec.iter().last().unwrap().created_at.to_owned();

            Some(ListStoryToken {
                sort_kind: SortKind::New,
                value: token_value,
            })
        } else {
            None
        };

        tracing::debug!("{rec:?}");

        Ok((rec, token))
    }

    async fn get_comment(&self, comment_id: i64) -> Result<Comment, StorageError> {
        let mut conn = self.pool.acquire().await?;

        // TODO: Add token filtering
        // TODO: These SQL queries can be combined
        let comment = sqlx::query(
            r#"
            SELECT id
                , author_id
                , story_id
                , parent_id
                , text
                , created_at
            FROM comments
            WHERE id = ?1
            "#,
        )
        .bind(comment_id)
        .map(|row: SqliteRow| Comment {
            id: row.get(0),
            author_id: row.get(1),
            story_id: row.get(2),
            parent_id: row.get(3),
            text: row.get(4),
            created_at: row.get(5),
        })
        .fetch_one(&mut conn)
        .await?;

        Ok(comment)
    }
}

const LIST_STORIES_NEW: &str = r#"
    SELECT id
        , title
        , author_id
        , url
        , text
        , created_at
    FROM stories
    -- Our token is a "CreatedAt" which may or may not be NULL
    WHERE created_at < ?1
        OR ?1 IS NULL
    ORDER BY created_at DESC
    LIMIT 25;
"#;

const LIST_COMMENTS_NEW: &str = r#"
    SELECT id
        , author_id
        , text
        , parent_id
        , created_at
    FROM comments
    WHERE story_id = ?1
        -- Our token is a "CreatedAt" which may or may not be NULL
        AND (created_at < ?2 OR ?2 IS NULL)
    ORDER BY created_at DESC
    LIMIT 50;
"#;
