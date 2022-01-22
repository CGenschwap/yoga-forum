use serde_json::json;
use yoga_forum::api::NewCommentResponse;
use yoga_forum::error::ErrorResponse;
mod common;

#[tokio::test]
async fn test_new_comment() {
    let client = reqwest::Client::new();
    let token = common::sign_in().await;

    let res = client
        .post(common::build_url("/stories/1/comments/new"))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "text": common::gen_rand_string(),
        }))
        .send()
        .await
        .unwrap();

    let res = res.json::<NewCommentResponse>().await;
    assert!(res.is_ok());

    let res_2 = client
        .post(common::build_url("/stories/1/comments/new"))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "text": common::gen_rand_prefix_string("child"),
            "parent_id": res.unwrap().comment_id,
        }))
        .send()
        .await
        .unwrap();

    let res_2 = res_2.json::<NewCommentResponse>().await;
    assert!(res_2.is_ok());
}

#[tokio::test]
async fn test_new_comment_invalid_parent() {
    let client = reqwest::Client::new();
    let token = common::sign_in().await;

    let res = client
        .post(common::build_url("/stories/1/comments/new"))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "text": common::gen_rand_string(),
            "parent_id": i64::MAX,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 404);
    let res = res.json::<ErrorResponse>().await.unwrap();
    assert_eq!(res.error_code, "INVALID_STORY_OR_PARENT_ID");
}
