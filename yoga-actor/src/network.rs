use anyhow::Result;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use yoga_forum::api::{
    ListCommentsResponse, ListStoriesResponse, NewCommentResponse, NewStoryResponse,
};
use yoga_forum::auth::api::{LoginResponse, NewUserResponse};

pub const BASE_URL: &str = "http://0.0.0.0:8080/v1/api";

fn build_url(a: &str) -> String {
    format!("{BASE_URL}{a}")
}

pub async fn sign_in(user_id: i64) -> Result<String> {
    let client = reqwest::Client::new();

    let url = build_url("/users/login");
    let res = client
        .post(url)
        .json(&serde_json::json!({
            "user_id": user_id,
            "password": "Testing",
        }))
        .send()
        .await?;
    let res = res.json::<LoginResponse>().await?;
    let res = res.token.as_str().to_owned();
    Ok(res)
}

pub async fn create_user() -> Result<i64> {
    let client = reqwest::Client::new();

    let url = build_url("/users/new");
    let res = client
        .post(url)
        .json(&serde_json::json!({
            "username": &gen_rand_prefix_string("user"),
            "password": "Testing",
        }))
        .send()
        .await?;
    let res = res.json::<NewUserResponse>().await?;
    Ok(res.user_id)
}

fn _gen_rand_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}

fn gen_rand_prefix_string(prefix: &str) -> String {
    format!(
        "{}-{}",
        prefix,
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect::<String>()
    )
}

pub async fn load_stories(next_token: Option<&str>) -> Result<ListStoriesResponse> {
    let client = reqwest::Client::new();

    let req = client.get(build_url("/stories"));
    let req = if let Some(next_token) = next_token {
        req.query(&[("next_token", next_token)])
    } else {
        req
    };

    let res = req.send().await?;

    let res = res.json::<ListStoriesResponse>().await?;
    Ok(res)
}

pub async fn load_comments(
    story_id: i64,
    next_token: Option<&str>,
) -> Result<ListCommentsResponse> {
    let client = reqwest::Client::new();

    let req = client.get(build_url(&format!("/stories/{story_id}/comments")));
    let req = if let Some(next_token) = next_token {
        req.query(&[("next_token", next_token)])
    } else {
        req
    };

    let res = req.send().await?;

    let res = res.json::<ListCommentsResponse>().await?;
    Ok(res)
}

pub async fn create_story(token: &str) -> Result<i64> {
    let client = reqwest::Client::new();

    let url = build_url("/stories/new");
    let res = client
        .post(url)
        .bearer_auth(token)
        .json(&serde_json::json!({
            "title": &gen_rand_prefix_string("title"),
            "url": &gen_rand_prefix_string("http://example.com/"),
            "text": &gen_rand_prefix_string("body"),
        }))
        .send()
        .await?;
    let res = res.json::<NewStoryResponse>().await?;
    Ok(res.story_id)
}

pub async fn create_comment(token: &str, story_id: i64, parent_id: Option<i64>) -> Result<i64> {
    let client = reqwest::Client::new();

    let url = build_url(&format!("/stories/{story_id}/comments/new"));
    let res = client
        .post(url)
        .bearer_auth(token)
        .json(&serde_json::json!({
            "text": &gen_rand_prefix_string("comment"),
            "parent_id": parent_id,
        }))
        .send()
        .await?;
    let res = res.json::<NewCommentResponse>().await?;
    Ok(res.comment_id)
}
