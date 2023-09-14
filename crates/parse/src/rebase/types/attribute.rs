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
