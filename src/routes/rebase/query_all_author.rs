use actix_web::web;
use actix_web::HttpResponse;
use aion_types::rebase::response::ListAllItemsResponse;
use serde::Serialize;
use sqlx::query_as;
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub struct AuthorsResponse {
    pub author: Vec<String>,
}

#[tracing::instrument(
    name = "Querying all authors",
    skip(conn_pool),
    fields(db_table = "new_rebase_daily")
)]
pub async fn list_authors(conn_pool: web::Data<PgPool>) -> HttpResponse {
    // Execute the database query
    let result = query_as!(
        ListAllItemsResponse,
        "SELECT id, hash, author, episode, introduce, time, title, url, tag FROM new_rebase_daily",
    )
    .fetch_all(conn_pool.as_ref())
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
            HttpResponse::Ok().json(AuthorsResponse { author: authors })
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
