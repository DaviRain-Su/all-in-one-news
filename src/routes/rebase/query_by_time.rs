use actix_web::web;
use actix_web::HttpResponse;
use aion_types::rebase::response::ListAllItemsResponse;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use sqlx::query_as;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeQuery {
    #[serde(
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime"
    )]
    pub time: DateTime<Utc>,
}

// 自定义日期时间序列化函数
fn serialize_datetime<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = datetime.to_rfc3339();
    serializer.serialize_str(&formatted)
}

// 自定义日期时间反序列化函数
fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(s)
        .map_err(serde::de::Error::custom)
        .map(|dt| dt.with_timezone(&Utc))
}

#[tracing::instrument(
    name = "Query items by time",
    skip(query_params, conn_poll),
    fields(
        time = %query_params.time
    )
)]
pub async fn list_by_time(
    query_params: web::Form<TimeQuery>,
    conn_poll: web::Data<PgPool>,
) -> HttpResponse {
    let tags_result = query_as!(
            ListAllItemsResponse,
            "SELECT id, hash,  author, episode, introduce, time, title, url FROM rebase_daily WHERE time = $1",
            &query_params.time
        )
        .fetch_all(conn_poll.as_ref())
        .await;

    match tags_result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
