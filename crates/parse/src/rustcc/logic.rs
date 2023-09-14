use super::types::section_link::SectionLink;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

pub async fn run() -> anyhow::Result<()> {
    let cpu_count = num_cpus::get();
    let task_count = 66; // Total tasks to be processed
    let tasks_per_thread = task_count / cpu_count;

    let mut tasks = vec![];

    for i in 0..cpu_count {
        let start = i * tasks_per_thread;
        let end = if i == cpu_count - 1 {
            task_count - 1
        } else {
            (i + 1) * tasks_per_thread - 1
        };
        let file = File::create(format!("target/rustcc_{}.json", start)).unwrap();
        let file1 = Arc::new(Mutex::new(file));

        file1.lock().unwrap().write_all("[".as_bytes())?;

        let file = file1.clone();

        let task = tokio::spawn(async move {
            process_task_range(start, end, file).await;
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    for i in 0..cpu_count {
        let start = i * tasks_per_thread;
        let mut file = File::open(format!("target/rustcc_{}.json", start)).unwrap();
        file.write_all("]".as_bytes())?;
    }

    Ok(())
}

pub async fn process_task_range(start: usize, end: usize, file: Arc<Mutex<File>>) {
    for idx in start..=end {
        let section_link = SectionLink { id: idx };

        if let Ok(article_list) = section_link.get_articles().await {
            for article in article_list.article_list {
                if let Ok(messages) = article.content().await {
                    let json_v = super::display_notion::convert_to_json_value(&messages);
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

async fn load_single(idx: usize) -> anyhow::Result<()> {
    let file = File::create(format!("target/rustcc_{}.json", idx)).unwrap();
    let file1 = Arc::new(Mutex::new(file));

    file1.lock().unwrap().write_all("[".as_bytes()).unwrap();

    let start = idx;
    let end = idx;

    process_task_range(start, end, file1.clone()).await;

    file1.lock().unwrap().write_all("]".as_bytes()).unwrap();

    Ok(())
}

pub async fn wrap_run_signle() -> anyhow::Result<()> {
    for idx in 61..=66 {
        if load_single(idx).await.is_ok() {
            println!(
                "---------------------------section {} successful load!----------------",
                idx
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_local_single() {
    let r = load_single(5).await;
    println!("r = {:?}", r);
}
