use serde_json::json;
use yoga_forum::api::NewStoryResponse;
use yoga_forum::error::ErrorResponse;
mod common;

#[tokio::test]
async fn test_new_story() {
    let client = reqwest::Client::new();
    let token = common::sign_in().await;

    let res = client
        .post(common::build_url("/stories/new"))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "title": common::gen_rand_prefix_string("title"),
            "text": common::gen_rand_prefix_string("text"),
            "url": "https://test.com",
        }))
        .send()
        .await
        .unwrap();

    let res = res.json::<NewStoryResponse>().await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_new_story_only_title() {
    let client = reqwest::Client::new();
    let token = common::sign_in().await;

    let res = client
        .post(common::build_url("/stories/new"))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "title": common::gen_rand_prefix_string("title"),
        }))
        .send()
        .await
        .unwrap();

    let res = res.json::<NewStoryResponse>().await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_new_story_no_title() {
    let client = reqwest::Client::new();
    let token = common::sign_in().await;

    let res = client
        .post(common::build_url("/stories/new"))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "text": "Some text",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    let res = res.json::<ErrorResponse>().await.unwrap();
    assert_eq!(res.error_code, "INVALID_REQUEST");
}

#[tokio::test]
async fn test_new_story_invalid_url() {
    let client = reqwest::Client::new();
    let token = common::sign_in().await;

    let res = client
        .post(common::build_url("/stories/new"))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "title": "Here is a title",
            "text": "Some sort of text",
            "url": "https://test  .com",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 400);
    let res = res.json::<ErrorResponse>().await.unwrap();
    assert_eq!(res.error_code, "INVALID_REQUEST");
}
