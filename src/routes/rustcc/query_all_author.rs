use crate::routes::DatabaseConnection;
use aion_types::rebase::response::ListAllItemsResponse;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use sqlx::query_as;
use sqlx::Acquire;

#[derive(Debug, Serialize)]
pub struct AuthorsResponse {
    pub author: Vec<String>,
}

pub async fn list_authors(
    DatabaseConnection(mut conn_pool): DatabaseConnection,
) -> impl IntoResponse {
    let connection_pool = conn_pool
        .acquire()
        .await
        .expect("Failed to acquire connection");

    // Execute the database query
    let result = query_as!(
        ListAllItemsResponse,
        "SELECT id, author, episode, introduce, time, title, url, tag FROM new_rustcc_daily",
    )
    .fetch_all(connection_pool.as_mut())
    .await;

    match result {
        Ok(items) => {
            // TODO: don't work
            let re = regex::Regex::new(r"\b\n").unwrap();
            let mut authors: Vec<String> = Vec::new();
            for item in items {
                let author = re.replace_all(&item.author, " ").to_string();
                authors.push(author);
            }
            authors.sort();
            authors.dedup();
            Ok(Json(AuthorsResponse { author: authors }))
        }
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
