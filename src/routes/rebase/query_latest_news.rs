use crate::routes::DatabaseConnection;
use aion_types::rebase::response::ListAllItemsResponse;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::query_as;
use sqlx::Acquire;

pub async fn list_latest_news(
    DatabaseConnection(mut conn_pool): DatabaseConnection,
) -> impl IntoResponse {
    let connection_pool = conn_pool
        .acquire()
        .await
        .expect("Failed to acquire connection");

    let tags_result = query_as!(
           ListAllItemsResponse,
           "SELECT id, author, episode, introduce, time, title, url, tag FROM new_rebase_daily ORDER BY time DESC LIMIT 10",
       )
       .fetch_all(connection_pool.as_mut())
       .await;

    match tags_result {
        Ok(items) => Ok(Json(items)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
