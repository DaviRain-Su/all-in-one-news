use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RebaseDaliyMeta {
    pub pagination: RebaseDaliyPagination,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RebaseDaliyPagination {
    pub page: usize,
    pub page_count: usize,
    pub page_size: usize,
    pub total: usize,
}
