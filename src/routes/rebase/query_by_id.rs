use actix_web::web;
use actix_web::HttpResponse;
use aion_types::rebase::response::ListAllItemsResponse;
use sqlx::query_as;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct IdQuery {
    pub id: i32,
}

#[tracing::instrument(
    name = "Querying items by id",
    skip(query_params, conn_pool),
    fields(
        id = %query_params.id,
    )
)]
pub async fn list_by_id(
    query_params: web::Form<IdQuery>,
    conn_pool: web::Data<PgPool>,
) -> HttpResponse {
    let tags_result = query_as!(
           ListAllItemsResponse,
           "SELECT id, hash, author, episode, introduce, time, title, url, tag FROM new_rebase_daily WHERE id = $1",
           query_params.id,
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
