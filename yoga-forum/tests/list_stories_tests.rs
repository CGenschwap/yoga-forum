use yoga_forum::api::ListStoriesResponse;

mod common;

#[tokio::test]
async fn test_list_stories() {
    let client = reqwest::Client::new();

    let res = client
        .get(common::build_url("/stories"))
        .send()
        .await
        .unwrap();

    let res = res.json::<ListStoriesResponse>().await;
    assert!(res.is_ok());
    let res = res.unwrap();

    assert!(res.stories.len() > 0);
}
