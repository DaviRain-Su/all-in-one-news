use crate::rebase::constant::REBASE_RPC_URL;
use reqwest::StatusCode;

// Purpose: Test the health check endpoint of the API
pub async fn health_check() -> anyhow::Result<StatusCode> {
    let response = reqwest::get(format!(
        "{}?pagination[page]=1&pagination[pageSize]=2",
        REBASE_RPC_URL
    ))
    .await?;
    println!("Status: {}", response.status());
    Ok(response.status())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        assert!(health_check().await.is_ok());
    }
}
