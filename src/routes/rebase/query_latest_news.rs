use actix_web::web;
use actix_web::HttpResponse;
use aion_types::rebase::response::ListAllItemsResponse;
use sqlx::query_as;
use sqlx::PgPool;

#[tracing::instrument(name = "Query latest news", skip(conn_pool))]
pub async fn list_latest_news(conn_pool: web::Data<PgPool>) -> HttpResponse {
    let tags_result = query_as!(
           ListAllItemsResponse,
           "SELECT id, hash, author, episode, introduce, time, title, url, tag FROM new_rebase_daily ORDER BY time DESC LIMIT 10",
       )
       .fetch_all(conn_pool.as_ref())
       .await;

    match tags_result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
