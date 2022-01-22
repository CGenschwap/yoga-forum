use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub const BASE_URL: &str = "http://0.0.0.0:8080/v1/api";

pub fn build_url(a: &str) -> String {
    format!("{BASE_URL}{a}")
}

pub async fn sign_in() -> String {
    let client = reqwest::Client::new();

    let url = build_url("/users/login");
    let res = client
        .post(url)
        .json(&serde_json::json!({
            "user_id": 1,
            "password": "Testing",
        }))
        .send()
        .await
        .unwrap();
    let res = res.json::<serde_json::Value>().await.unwrap();
    return res.get("token").unwrap().as_str().unwrap().to_owned();
}

pub fn gen_rand_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(50)
        .map(char::from)
        .collect()
}

pub fn gen_rand_prefix_string(prefix: &str) -> String {
    format!(
        "{}-{}",
        prefix,
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(50)
            .map(char::from)
            .collect::<String>()
    )
}
