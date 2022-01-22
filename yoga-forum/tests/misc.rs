mod common;

#[tokio::test]
async fn test_ping() {
    let pong = reqwest::get(common::build_url("/ping"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(pong, "Pong");

    let pong = reqwest::get(common::build_url("/ping"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(pong, "Pong");
}
