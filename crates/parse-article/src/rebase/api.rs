use super::types::RebaseDaliy;

pub async fn total_count() -> anyhow::Result<usize> {
    let response = reqwest::get(
        "https://db.rebase.network/api/v1/geekdailies?pagination[page]=1&pagination[pageSize]=1",
    )
    .await?;

    let body = response.text().await?;
    let json: RebaseDaliy = serde_json::from_str(&body)?;

    Ok(json.total_count())
}

#[tokio::test]
async fn test_total_count() {
    let count = total_count().await.unwrap();
    println!("total count: {}", count);
}
