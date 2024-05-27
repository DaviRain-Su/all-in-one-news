use actix_web::web;
use actix_web::HttpResponse;
use aion_types::rebase::response::ListAllItemsResponse;
use sqlx::query_as;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct ListAllItemsQuery {
    page: i64,
    per_page: i64,
}

pub async fn list_all_items(
    query: web::Form<ListAllItemsQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // Calculate the OFFSET and LIMIT based on the query parameters
    let offset = query.page * query.per_page;
    let limit = query.per_page;

    // Execute the database query
    let result = query_as!(
        ListAllItemsResponse,
        "SELECT id, hash, author, episode, introduce, time, title, url, tag FROM new_rebase_daily ORDER BY time DESC OFFSET $1 LIMIT $2",
        offset,
        limit
    )
    .fetch_all(pool.as_ref())
    .await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn list_all(pool: web::Data<PgPool>) -> HttpResponse {
    // Execute the database query
    let result = query_as!(
        ListAllItemsResponse,
        "SELECT id, hash, author, episode, introduce, time, title, url, tag FROM new_rebase_daily ORDER BY time DESC",
    )
    .fetch_all(pool.as_ref())
    .await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
