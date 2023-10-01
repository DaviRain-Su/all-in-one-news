use crate::routes::DatabaseConnection;
use aion_types::rebase::response::ListAllItemsResponse;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::query_as;
use sqlx::Acquire;

#[derive(serde::Deserialize)]
pub struct HashQuery {
    pub hash: String,
}

pub async fn list_by_hash(
    DatabaseConnection(mut conn_pool): DatabaseConnection,
    Query(query_params): Query<HashQuery>,
) -> impl IntoResponse {
    let connection_pool = conn_pool
        .acquire()
        .await
        .expect("Failed to acquire connection");

    let tags_result = query_as!(
           ListAllItemsResponse,
           "SELECT id, hash, author, episode, introduce, time, title, url, tag FROM new_rustcc_daily WHERE hash = $1",
           query_params.hash,
       )
       .fetch_all(connection_pool.as_mut())
       .await;

    match tags_result {
        Ok(items) => Ok(Json(items)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
