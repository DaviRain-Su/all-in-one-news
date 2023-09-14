//! tests/health_check.rs
use once_cell::sync::Lazy;
use sqlx::PgPool;
use sqlx::{Connection, Executor, PgConnection};
use wiremock::MockServer;

use all_in_one_news::configuration::{get_configuration, DatabaseSettings};
use uuid::Uuid;

use all_in_one_news::startup;
use all_in_one_news::startup::get_connection_pool;
use all_in_one_news::telemetry::{get_subscriber, init_subscriber};

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    // We cannot assign the output of `get_subscriber` to a variable based on the value
    // of `TEST_LOG` because the sink is part of the type returned by `get_subscriber`,
    // therefore they are not the same type. We could work around it, but this is the
    // most straight-forward way of moving forward.
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber).unwrap();
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber).unwrap();
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

// Launch our application in the background ~somehow~
pub async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    // launch a mock server to stand in for the email server
    let email_server = MockServer::start().await;

    let configuration = {
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        // use a different database for each test case
        configuration.database.database_name = Uuid::new_v4().to_string();
        // use a radom port
        configuration.application.port = 0;

        configuration
    };

    // create and migrate database
    configure_database(&configuration.database).await;

    let application = startup::Application::build(configuration.clone())
        .await
        .unwrap();
    let address = format!("http://127.0.0.1:{}", application.port());
    let _application = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
        email_server,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    // Migrate database
    PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.")
    // sqlx::migrate!("./migrations")
    // .run(&connection_pool)
    // .await
    // .expect("Failed to migrate the database");
    // connection_pool
}
