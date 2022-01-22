use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::json;
use yoga_forum::auth::api::NewUserResponse;
mod common;

#[tokio::test]
async fn test_new_user() {
    let client = reqwest::Client::new();
    let username = generate_username();

    let res = client
        .post(common::build_url("/users/new"))
        .json(&json!({
            "username": username,
            "password": "ThisIsNotATest",
        }))
        .send()
        .await
        .unwrap();

    let res = res.json::<NewUserResponse>().await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_new_user_duplicate() {
    let client = reqwest::Client::new();
    let username = generate_username();

    let res = client
        .post(common::build_url("/users/new"))
        .json(&json!({
            "username": username,
            "password": "ThisIsNotATest",
        }))
        .send()
        .await
        .unwrap();

    let res = res.json::<NewUserResponse>().await;
    assert!(res.is_ok());

    let res = client
        .post(common::build_url("/users/new"))
        .json(&json!({
            "username": username,
            "password": "ThisIsNotATest",
        }))
        .send()
        .await
        .unwrap();

    // Assert we have a CONFLICT status code and
    // that we have _some_ error message
    assert_eq!(res.status(), 409);
    assert!(res.text().await.unwrap().len() > 5);
}

fn generate_username() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}
