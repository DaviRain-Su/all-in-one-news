use crate::routes::DatabaseConnection;
use aion_types::rebase::response::ListAllItemsResponse;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::query_as;
use sqlx::Acquire;

#[derive(serde::Deserialize)]
pub struct ListAllItemsQuery {
    page: i64,
    per_page: i64,
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
        "SELECT id, author, episode, introduce, time, title, url, tag FROM new_rustcc_daily OFFSET $1 LIMIT $2",
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

pub async fn list_all(DatabaseConnection(mut conn_pool): DatabaseConnection) -> impl IntoResponse {
    let connection_pool = conn_pool
        .acquire()
        .await
        .expect("Failed to acquire connection");

    // Execute the database query
    let result = query_as!(
        ListAllItemsResponse,
        "SELECT id, author, episode, introduce, time, title, url, tag FROM new_rustcc_daily ORDER BY time DESC",
    )
    .fetch_all(connection_pool.as_mut())
    .await;

    match result {
        Ok(items) => Ok(Json(items)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
