//! Auth Tests
//!
//! TODO: The tests should be augmented to ensure that the error message is relevant and correct
//!
//! TODO: Add some tests around the edge-cases (tokens that should succeed)
//!
use serde_json::json;
use yoga_forum::error::ErrorResponse;
mod common;

#[tokio::test]
async fn test_bad_login() {
    let client = reqwest::Client::new();

    let res = client
        .post(common::build_url("/users/login"))
        .json(&serde_json::json!({
            "user_id": 1,
            "password": "wrong",
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
    let res = res.json::<ErrorResponse>().await.unwrap();
    assert_eq!(res.error_code, "INVALID_USERNAME_OR_PASS");
}

#[tokio::test]
async fn test_bad_encoding_token() {
    let client = reqwest::Client::new();
    let token = common::sign_in().await;

    // Using `/stories/new` but it doesn't really matter
    let res = client
        .post(common::build_url("/stories/new"))
        .header("Authorization", format!("Bearer z{}", token))
        .json(&json!({
            "title": "Here is a title",
            "text": "Some sort of text",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    let error_res = res.json::<ErrorResponse>().await;
    assert!(error_res.is_ok());

    // NOTE: We want to hard-code this value rather than using a shared
    // enum because it is API breakage to change error codes.
    let error_res = error_res.unwrap();
    assert_eq!(error_res.error_code, "INVALID_BEARER");
}

#[tokio::test]
async fn test_no_token() {
    let client = reqwest::Client::new();
    let _token = common::sign_in().await;

    // Using `/stories/new` but it doesn't really matter
    let res = client
        .post(common::build_url("/stories/new"))
        .json(&json!({
            "title": "Here is a title",
            "text": "Some sort of text",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    let error_res = res.json::<ErrorResponse>().await;
    assert!(error_res.is_ok());

    // NOTE: We want to hard-code this value rather than using a shared
    // enum because it is API breakage to change error codes.
    let error_res = error_res.unwrap();
    assert_eq!(error_res.error_code, "INVALID_BEARER");
}

#[tokio::test]
async fn test_invalid_token() {
    let client = reqwest::Client::new();
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";

    // Using `/stories/new` but it doesn't really matter
    let res = client
        .post(common::build_url("/stories/new"))
        .header("Authorization", format!("Bearer {}", token))
        .json(&json!({
            "title": "Here is a title",
            "text": "Some sort of text",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    let error_res = res.json::<ErrorResponse>().await;
    assert!(error_res.is_ok());

    // NOTE: We want to hard-code this value rather than using a shared
    // enum because it is API breakage to change error codes.
    let error_res = error_res.unwrap();
    assert_eq!(error_res.error_code, "INVALID_BEARER");
}

#[tokio::test]
async fn test_wrong_scheme_token() {
    let client = reqwest::Client::new();
    let token = common::sign_in().await;

    // Using `/stories/new` but it doesn't really matter
    let res = client
        .post(common::build_url("/stories/new"))
        .header("Authorization", format!("{}", token))
        .json(&json!({
            "title": "Here is a title",
            "text": "Some sort of text",
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), 401);
    let error_res = res.json::<ErrorResponse>().await;
    assert!(error_res.is_ok());

    // NOTE: We want to hard-code this value rather than using a shared
    // enum because it is API breakage to change error codes.
    let error_res = error_res.unwrap();
    assert_eq!(error_res.error_code, "INVALID_BEARER");
}
