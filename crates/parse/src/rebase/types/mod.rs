use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub mod attribute;
pub mod meta;

pub use attribute::RebaseDaliyAttribute;
pub use meta::RebaseDaliyMeta;

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

impl RebaseDaliy {
    pub fn convert_to_json_value(&self) -> Vec<Value> {
        let mut properties = Vec::new();

        for message in self.data.iter() {
            let title = json!({
                "type": "title",
                "title": [
                    {
                        "type": "text",
                        "text": {
                            "content": message.attributes.title
                        }
                    }
                ]
            });

            let author = json!({
                "type": "rich_text",
                "rich_text": [
                    {
                        "type": "text",
                        "text": {
                            "content": message.attributes.author
                        }
                    }
                ]
            });

            let episode = json!({
                "type": "rich_text",
                "rich_text": [
                    {
                        "type": "text",
                        "text": {
                            "content": message.attributes.episode
                        }
                    }
                ]
            });

            let time = json!({
                "type": "rich_text",
                "rich_text": [
                    {
                        "type": "text",
                        "text": {
                            "content": message.attributes.time
                        }
                    }
                ]
            });

            let link = json!({
                "type": "rich_text",
                "rich_text": [
                    {
                        "type": "text",
                        "text": {
                            "content": message.attributes.url
                        }
                    }
                ]
            });

            let intro = json!({
                "type": "rich_text",
                "rich_text": [
                    {
                        "type": "text",
                        "text": {
                            "content": message.attributes.introduce
                        }
                    }
                ]
            });

            let message_json = json!({
                "Title": title,
                "Author": author,
                "Intro": intro,
                "Episode": episode,
                "Time": time,
                "Link": link,
            });
            properties.push(message_json)
        }

        properties
    }

    pub fn total_count(&self) -> usize {
        self.meta.pagination.total
    }
}
