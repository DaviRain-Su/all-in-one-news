use serde::{Deserialize, Serialize};

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
