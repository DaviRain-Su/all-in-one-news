pub mod types;

use types::ListAllItemsResponse;

pub async fn get_latest_news() -> anyhow::Result<Vec<ListAllItemsResponse>> {
    let resonse = reqwest::get("http://127.0.0.1:8000/latest")
        .await?
        .text()
        .await?;

    let rebase_daily: Vec<ListAllItemsResponse> = serde_json::from_str(&resonse)?;
    Ok(rebase_daily)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_latest_news() {
        let result = get_latest_news().await.unwrap();
        println!("result: {:?}", result);
    }
}
