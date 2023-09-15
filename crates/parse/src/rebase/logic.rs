use crate::rebase::constant::REBASE_RPC_URL;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use super::types::RebaseDaliy;
use crate::rebase::types::RebaseDaliyEpisode;

pub async fn process_task_range(start: usize, end: usize, file: Arc<Mutex<File>>) {
    for id in start..=end {
        let html = reqwest::get(format!(
            "{}?pagination[page]={}&pagination[pageSize]=1",
            REBASE_RPC_URL, id
        ))
        .await;
        if let Ok(html) = html {
            if let Ok(body) = html.text().await {
                if let Ok(rebase_daily) = serde_json::from_str::<RebaseDaliy>(&body) {
                    let json_v = rebase_daily.convert_to_json_value();
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

pub async fn parse_rebase_data(start: usize, end: usize) -> Vec<RebaseDaliyEpisode> {
    let mut result = vec![];
    for id in start..=end {
        let html = reqwest::get(format!(
            "{}?pagination[page]={}&pagination[pageSize]=1",
            REBASE_RPC_URL, id
        ))
        .await;
        if let Ok(html) = html {
            if let Ok(body) = html.text().await {
                if let Ok(rebase_daily) = serde_json::from_str::<RebaseDaliy>(&body) {
                    println!("rebase_daily: {:?}", rebase_daily);
                    result.push(rebase_daily);
                }
            }
        }
    }

    // result
    result.into_iter().flat_map(|item| item.data).collect()
}
