use super::types::RebaseDaliy;
use serde_json::{json, Value};

pub fn convert_to_json_value(notion_page: &RebaseDaliy) -> Vec<Value> {
    let mut properties = Vec::new();

    for message in notion_page.data.iter() {
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
