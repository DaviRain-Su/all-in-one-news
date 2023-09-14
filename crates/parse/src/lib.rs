pub mod rebase;
pub mod rustcc;

use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

pub async fn load_all_rebase_source() -> anyhow::Result<()> {
    let file = File::create("target/rebase_all.json").unwrap();
    let file1 = Arc::new(Mutex::new(file));

    file1.lock().unwrap().write_all("[".as_bytes())?;

    let cpu_count = num_cpus::get();
    let task_count = rebase::api::total_count().await?; // Total tasks to be processed
                                                        // let task_count = 1000;
    let tasks_per_thread = task_count / cpu_count;

    let mut tasks = vec![];

    for i in 0..cpu_count {
        let start = i * tasks_per_thread;
        let end = if i == cpu_count - 1 {
            task_count - 1
        } else {
            (i + 1) * tasks_per_thread - 1
        };

        let file = file1.clone();

        let task = tokio::spawn(async move {
            rebase::logic::process_task_range(start, end, file).await;
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    file1.lock().unwrap().write_all("]".as_bytes())?;

    Ok(())
}

pub async fn load_rebase_single() -> anyhow::Result<()> {
    let file = File::create("target/latest.json").unwrap();
    let file1 = Arc::new(Mutex::new(file));

    file1.lock().unwrap().write_all("[".as_bytes()).unwrap();

    let before_totall_count = 4143;

    let task_count = rebase::api::total_count().await?;

    let start = before_totall_count + 1;
    let end = task_count;

    rebase::logic::process_task_range(start, end, file1.clone()).await;

    file1.lock().unwrap().write_all("]".as_bytes()).unwrap();

    Ok(())
}
