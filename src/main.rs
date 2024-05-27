use all_in_one_news::configuration::get_configuration;
use all_in_one_news::startup::Application;

use all_in_one_news::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("aion".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber)?;

    let configuration = get_configuration()?;

    let application = Application::build(configuration.clone()).await?;
    println!(
        "🌟🌟🌟🌟🌟🌟 Server is running on port {} 🌟🌟🌟🌟🌟",
        application.port()
    );

    let _ = application.run_until_stopped().await;

    Ok(())
}
