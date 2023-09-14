use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RebaseDaliyAttribute {
    pub author: String,
    pub episode: String,
    pub introduce: String,
    pub time: String,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RebaseDaliyEpisode {
    pub attributes: RebaseDaliyAttribute,
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RebaseDaliy {
    pub data: Vec<RebaseDaliyEpisode>,
    pub meta: RebaseDaliyMeta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RebaseDaliyMeta {
    pub pagination: RebaseDaliyPagination,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RebaseDaliyPagination {
    pub page: usize,
    #[serde(rename = "pageCount")]
    pub page_count: usize,
    #[serde(rename = "pageSize")]
    pub page_size: usize,
    pub total: usize,
}

impl RebaseDaliy {
    pub fn total_count(&self) -> usize {
        self.meta.pagination.total
    }
}
