// Define rebase api
use crate::rebase::types::ListAllItemsResponse;

pub static BASE_API_URL: &str = "http://127.0.0.1:8000";

pub async fn get_latest_new() -> anyhow::Result<Vec<ListAllItemsResponse>> {
    let url = format!("{}/latest", BASE_API_URL);
    reqwest::get(&url).await?.json().await.map_err(|e| e.into())
}

pub async fn get_new_by_id(id: i32) -> anyhow::Result<ListAllItemsResponse> {
    let url = format!("{}/by_id?id={}", BASE_API_URL, id);
    let rebase_daily = reqwest::get(&url)
        .await?
        .json::<Vec<ListAllItemsResponse>>()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    assert_eq!(rebase_daily.len(), 1);

    Ok(rebase_daily.first().unwrap().clone())
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_get_new_by_id() {
        let story = crate::fetch_data::get_new_by_id(4372).await.unwrap();

        println!("{:?}", story);
    }

    #[tokio::test]
    async fn test_get_latest_new() {
        let stories = crate::fetch_data::get_latest_new().await.unwrap();
        println!("{:?}", stories);
    }
}
