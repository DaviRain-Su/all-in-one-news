use crate::templates::latest_news::LatestNewsTemplate;
use actix_web::web;
use actix_web::HttpResponse;
use aion_types::rebase::response::ListAllItemsResponse;
use aion_types::rebase::response::SimpleDisplay;
use askama_actix::TemplateToResponse;
use sqlx::query_as;
use sqlx::PgPool;

#[tracing::instrument(name = "Query latest news", skip(conn_pool))]
pub async fn list_latest_news(conn_pool: web::Data<PgPool>) -> HttpResponse {
    let tags_result = query_as!(
           ListAllItemsResponse,
           "SELECT id, hash, author, episode, introduce, time, title, url FROM rebase_daily ORDER BY time DESC LIMIT 20",
       )
       .fetch_all(conn_pool.as_ref())
       .await;

    match tags_result {
        Ok(items) => {
            let items = items
                .into_iter()
                .map(SimpleDisplay::from)
                .collect::<Vec<_>>();
            let template = LatestNewsTemplate { items };
            template.to_response()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
