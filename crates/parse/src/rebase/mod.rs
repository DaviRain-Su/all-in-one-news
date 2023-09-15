pub mod api;
pub mod constant;
pub mod hellath_check;
pub mod logic;
pub mod types;

use crate::rebase::types::RebaseDaliyEpisode;

/// get total rebase daily episode
pub async fn get_total_rebase_daily_episode() -> anyhow::Result<Vec<RebaseDaliyEpisode>> {
    let start = 1;
    let end = api::total_count().await?;
    Ok(logic::parse_rebase_data(start, end).await)
}
