use actix_web::web;
use actix_web::HttpResponse;
use aion_types::rebase::response::ListAllItemsResponse;
use sqlx::query_as;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct TagsQuery {
    pub tag: String,
}

pub async fn list_tags(
    query_params: web::Form<TagsQuery>,
    conn_pool: web::Data<PgPool>,
) -> HttpResponse {
    let tags_result = query_as!(
            ListAllItemsResponse,
            "SELECT id, hash, author, episode, introduce, time, title, url, tag FROM new_rebase_daily WHERE $1 = ANY(tag)",
            &query_params.tag
        )
        .fetch_all(conn_pool.as_ref())
        .await;

    match tags_result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
