use reqwest::StatusCode;

// Purpose: Test the health check endpoint of the API
pub async fn health_check() -> anyhow::Result<StatusCode> {
    let response = reqwest::get(
        "https://db.rebase.network/api/v1/geekdailies?pagination[page]=1&pagination[pageSize]=2",
    )
    .await?;
    println!("Status: {}", response.status());
    Ok(response.status())
}

#[tokio::test]
async fn test_health_check() {
    assert!(health_check().await.is_ok());
}
