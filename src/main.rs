#![allow(clippy::await_holding_lock)]

use all_in_one_news::configuration::get_configuration;
use all_in_one_news::startup::{get_connection_pool, process_load_all_rebase_daily, Application};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aion=trace,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let configuration = get_configuration()?;

    // let pg_pool = get_connection_pool(&configuration.database);
    // let pg = Arc::new(pg_pool);

    // process_load_all_rebase_daily(pg).await?;

    let service = Application::build(configuration.clone()).await?;
    println!("🌟🌟🌟🌟🌟🌟 Server is running on port 8000 🌟🌟🌟🌟🌟");

    let _ = service.run_until_stopped().await;

    Ok(())
}
