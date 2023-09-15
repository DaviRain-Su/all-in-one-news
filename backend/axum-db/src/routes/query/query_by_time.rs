use crate::routes::query::query_all::ListAllItemsResponse;
use crate::routes::DatabaseConnection;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use sqlx::query_as;
use sqlx::Acquire;

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

pub async fn list_by_time(
    DatabaseConnection(mut conn_pool): DatabaseConnection,
    Query(query_params): Query<TimeQuery>,
) -> impl IntoResponse {
    let connection_pool = conn_pool
        .acquire()
        .await
        .expect("Failed to acquire connection");

    let tags_result = query_as!(
            ListAllItemsResponse,
            "SELECT id, author, episode, introduce, time, title, url, tag FROM new_rebase_daily WHERE time = $1",
            &query_params.time
        )
        .fetch_all(connection_pool.as_mut())
        .await;

    match tags_result {
        Ok(items) => Ok(Json(items)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
