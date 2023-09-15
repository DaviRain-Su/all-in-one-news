use crate::routes::DatabaseConnection;
use axum::extract::Query;
use axum::Json;
use sqlx::query_as;
use sqlx::Acquire;

#[derive(serde::Deserialize)]
pub struct ListAllItemsQuery {
    page: i64,
    per_page: i64,
}

#[derive(Debug)]
pub struct ListAllItemsResponse {
    pub author: String,
    pub episode: String,
    pub introduce: String,
    pub time: chrono::DateTime<chrono::Utc>,
    pub title: String,
    pub url: String,
}

pub async fn list_all_items(
    DatabaseConnection(mut conn_pool): DatabaseConnection,
    Query(query): Query<ListAllItemsQuery>,
) -> Result<Json<Vec<ListAllItemsResponse>>, axum::http::StatusCode> {
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
