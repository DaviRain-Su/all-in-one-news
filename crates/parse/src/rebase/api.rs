use crate::rebase::constant::REBASE_RPC_URL;
use crate::rebase::types::RebaseDaliy;

#[tracing::instrument(name = "Get total count of rebase daily")]
pub async fn total_count() -> anyhow::Result<usize> {
    let response = reqwest::get(format!(
        "{}?pagination[page]=1&pagination[pageSize]=1",
        REBASE_RPC_URL
    ))
    .await?;

    let body = response.text().await?;
    let json: RebaseDaliy = serde_json::from_str(&body)?;

    Ok(json.total_count())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    async fn test_total_count() {
        let count = total_count().await.unwrap();
        println!("total count: {}", count);
    }
}
