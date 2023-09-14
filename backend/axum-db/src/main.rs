use all_in_one_news::configuration::get_configuration;
use all_in_one_news::startup::Application;
use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aion=trace,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let configuration = get_configuration()?;

    let service = Application::build(configuration).await?;
    println!("🌟🌟🌟🌟🌟🌟 Server is running on port 8000 🌟🌟🌟🌟🌟");

    service.run_until_stopped().await?;

    Ok(())
}
