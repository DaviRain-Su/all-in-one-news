use chrono::DateTime;
use chrono::Utc;
use serde::Deserializer;
use serde::{Deserialize, Serialize, Serializer};

// 自定义序列化器
fn serialize_datetime<S>(datetime: &DateTime<chrono::Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 将 DateTime<Utc> 格式化为 RFC3339 格式的字符串
    let formatted = datetime.to_rfc3339();

    // 调用 Serializer 的 `serialize_str` 方法将字符串序列化为 JSON 字符串
    serializer.serialize_str(&formatted)
}

fn serialize_id_as_num<S>(id: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i32(*id)
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AIonResponse {
    #[serde(
            serialize_with = "serialize_id_as_num" // 序列化时输出为数字
        )]
    pub id: i32,
    pub author: String,
    pub episode: String,
    pub introduce: String,
    #[serde(
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime"
    )]
    pub time: DateTime<chrono::Utc>,
    pub title: String,
    pub url: String,
    pub tag: Vec<String>,
}
