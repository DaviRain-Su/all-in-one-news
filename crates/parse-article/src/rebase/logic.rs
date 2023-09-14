use super::display_notion;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

pub async fn process_task_range(start: usize, end: usize, file: Arc<Mutex<File>>) {
    for id in start..=end {
        let html = reqwest::get(format!("https://db.rebase.network/api/v1/geekdailies?pagination[page]={}&pagination[pageSize]=1", id)).await;
        if let Ok(html) = html {
            if let Ok(body) = html.text().await {
                if let Ok(rebase_daily) = serde_json::from_str(&body) {
                    let json_v = display_notion::convert_to_json_value(&rebase_daily);
                    for msg in json_v.iter() {
                        println!("{}", serde_json::to_string_pretty(&msg).unwrap());
                        let json_data = msg.to_string();

                        file.lock()
                            .unwrap()
                            .write_all(json_data.to_string().as_bytes())
                            .unwrap();
                        file.lock().unwrap().write_all(",".as_bytes()).unwrap();
                    }
                }
            }
        }
    }
}
