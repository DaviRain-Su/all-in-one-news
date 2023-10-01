use crate::routes::DatabaseConnection;
use aion_types::rebase::response::ListAllItemsResponse;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::query_as;
use sqlx::Acquire;

pub async fn list_latest_news_ids(
    DatabaseConnection(mut conn_pool): DatabaseConnection,
) -> impl IntoResponse {
    let connection_pool = conn_pool
        .acquire()
        .await
        .expect("Failed to acquire connection");

    let result = query_as!(
        ListAllItemsResponse,
        "SELECT id, hash, author, episode, introduce, time, title, url, tag FROM new_rebase_daily",
    )
    .fetch_all(connection_pool.as_mut())
    .await;

    match result {
        Ok(items) => {
            let ids = items.into_iter().map(|item| item.id).collect::<Vec<i32>>();
            Ok(Json(ids))
        }
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
