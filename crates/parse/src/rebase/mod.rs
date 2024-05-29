pub mod api;
pub mod constant;
pub mod hellath_check;
pub mod logic;
pub mod types;

use crate::rebase::types::RebaseDaliyEpisode;

/// get total rebase daily episode
#[tracing::instrument(name = "Get total rebase daily episode")]
pub async fn get_total_rebase_daily_episode() -> anyhow::Result<Vec<RebaseDaliyEpisode>> {
    let cpu_count = num_cpus::get();
    let task_count = api::total_count().await?; // Total tasks to be processed

    let tasks_per_thread = task_count / cpu_count;

    let mut tasks = vec![];

    let mut reuslt_rebase_daily_episode = vec![];

    for i in 0..cpu_count {
        let start = i * tasks_per_thread;
        let end = if i == cpu_count - 1 {
            task_count - 1
        } else {
            (i + 1) * tasks_per_thread - 1
        };

        let task = tokio::spawn(async move { logic::parse_rebase_data(start, end).await });

        tasks.push(task);
    }

    for task in tasks {
        let mut result = task.await?;
        reuslt_rebase_daily_episode.append(&mut result);
    }

    // Sort by id
    reuslt_rebase_daily_episode.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(reuslt_rebase_daily_episode)
}

/// get total rebase daily ids
pub async fn get_total_rebase_daily_ids() -> anyhow::Result<Vec<usize>> {
    let result = get_total_rebase_daily_episode().await?;
    Ok(result.into_iter().map(|item| item.id).collect())
}
