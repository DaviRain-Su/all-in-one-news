use super::types::message::Messages;
use serde_json::{json, Value};

pub fn convert_to_json_value(messages: &Messages) -> Vec<Value> {
    let mut properties = Vec::new();

    for message in messages.messages.iter() {
        let title = json!({
            "type": "title",
            "title": [
                {
                    "type": "text",
                    "text": {
                        "content": message.title
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
                        "content": message.author
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
                        "content": message.time
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
                        "content": message.link
                    }
                }
            ]
        });

        let content = if message.to_string().len() > 2000 {
            message.to_string().chars().take(1990).collect::<String>()
        } else {
            message.to_string()
        };
        let intro = json!({
            "type": "rich_text",
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": content
                    }
                }
            ]
        });

        let message_json = json!({
            "Title": title,
            "Author": author,
            "Intro": intro,
            "Time": time,
            "Link": link,
        });
        properties.push(message_json)
    }

    properties
}
