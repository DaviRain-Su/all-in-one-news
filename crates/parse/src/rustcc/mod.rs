use self::types::message::Message;

pub mod display_notion;
pub mod health_check;
pub mod logic;
pub mod types;

use crate::rustcc::types::section_link::SectionLink;

/// get total rebase daily episode
pub async fn get_total_rustcc_daily_episode() -> anyhow::Result<Vec<Message>> {
    let cpu_count = num_cpus::get();
    let section_link = SectionLink { id: 1 };
    let task_count = section_link.totoal_page().await?; // Total tasks to be processed

    let tasks_per_thread = task_count / cpu_count;

    let mut tasks = vec![];

    let mut reuslt_rustcc_daily_episode = vec![];

    for i in 0..cpu_count {
        let start = i * tasks_per_thread;
        let end = if i == cpu_count - 1 {
            task_count - 1
        } else {
            (i + 1) * tasks_per_thread - 1
        };

        let task = tokio::spawn(async move { logic::parse_rustcc_data(start, end).await });

        tasks.push(task);
    }

    for task in tasks {
        let mut result = task.await?;
        reuslt_rustcc_daily_episode.append(&mut result);
    }

    Ok(reuslt_rustcc_daily_episode)
}

#[tokio::test]
async fn test_get_total_rustcc_daily_episode() {
    let result = get_total_rustcc_daily_episode().await.unwrap();
    println!("{:?}", result);
}
