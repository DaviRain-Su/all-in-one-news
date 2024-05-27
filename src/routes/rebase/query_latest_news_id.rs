use actix_web::web;
use actix_web::HttpResponse;
use aion_types::rebase::response::ListAllItemsResponse;
use sqlx::query_as;
use sqlx::PgPool;

#[tracing::instrument(
    name = "Query latest news ids",
    skip(connection_pool),
    fields(service = "rebase", region = "asia")
)]
pub async fn list_latest_news_ids(connection_pool: web::Data<PgPool>) -> HttpResponse {
    let result = query_as!(
        ListAllItemsResponse,
        "SELECT id, hash, author, episode, introduce, time, title, url, tag FROM new_rebase_daily",
    )
    .fetch_all(connection_pool.as_ref())
    .await;

    match result {
        Ok(items) => {
            let ids = items.into_iter().map(|item| item.id).collect::<Vec<i32>>();
            HttpResponse::Ok().json(ids)
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
