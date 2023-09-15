use crate::routes::DatabaseConnection;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;
use chrono::DateTime;
use serde::{Serialize, Serializer};
use sqlx::query_as;
use sqlx::Acquire;

#[derive(serde::Deserialize)]
pub struct ListAllItemsQuery {
    page: i64,
    per_page: i64,
}

// 自定义序列化器
fn serialize_datetime<S>(datetime: &DateTime<chrono::Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 将 DateTime<Utc> 格式化为 RFC3339 格式的字符串
    let formatted = datetime.to_rfc3339();

    // 调用 Serializer 的 `serialize_str` 方法将字符串序列化为 JSON 字符串
    serializer.serialize_str(&formatted)
}

#[derive(Debug, Serialize)]
pub struct ListAllItemsResponse {
    pub author: String,
    pub episode: String,
    pub introduce: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub time: DateTime<chrono::Utc>,
    pub title: String,
    pub url: String,
}

pub async fn list_all_items(
    DatabaseConnection(mut conn_pool): DatabaseConnection,
    Query(query): Query<ListAllItemsQuery>,
) -> impl IntoResponse {
    let connection_pool = conn_pool
        .acquire()
        .await
        .expect("Failed to acquire connection");
    // Calculate the OFFSET and LIMIT based on the query parameters
    let offset = query.page * query.per_page;
    let limit = query.per_page;

    // Execute the database query
    let result = query_as!(
        ListAllItemsResponse,
        "SELECT author, episode, introduce, time, title, url FROM new_rebase_daily OFFSET $1 LIMIT $2",
        offset,
        limit
    )
    .fetch_all(connection_pool.as_mut())
    .await;

    match result {
        Ok(items) => Ok(Json(items)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
