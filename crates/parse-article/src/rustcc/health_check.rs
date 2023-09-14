use reqwest::StatusCode;

pub async fn health_check() -> anyhow::Result<StatusCode> {
    let response =
        reqwest::get("https://rustcc.cn/article?id=1ad7d23c-2392-4cce-9dc7-4bebcb3d51a5").await?;

    Ok(response.status())
}

#[tokio::test]
async fn test_health_check() {
    let status = health_check().await.unwrap();
    assert_eq!(status, StatusCode::OK);
}
