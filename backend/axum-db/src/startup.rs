use crate::configuration::{DatabaseSettings, Settings};
use anyhow::Result;
use axum::routing::IntoMakeService;
use axum::Server;
use axum::{routing::get, Router};
use hyper::server::conn::AddrIncoming;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::routes::greet;
use crate::routes::health_check;
use crate::routes::index;

#[derive(Debug)]
pub struct Application {
    pub port: u16,
    pub server: Server<AddrIncoming, IntoMakeService<Router>>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let server = run(
            listener,
            connection_pool,
            // new argument from configuration
            configuration.application.base_url,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<()> {
        self.server.await.map_err(|e| anyhow::anyhow!(e))
    }
}

pub fn get_connection_pool(database_configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(database_configuration.with_db())
}

// ref: axum: https://github.com/tokio-rs/axum/blob/main/examples/oauth/src/main.rs#L78
#[derive(Clone)]
struct AppState {
    database: PgPool,
    base_url: ApplicationBaseUrl,
}

impl axum::extract::FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.database.clone()
    }
}

impl axum::extract::FromRef<AppState> for ApplicationBaseUrl {
    fn from_ref(state: &AppState) -> Self {
        state.base_url.clone()
    }
}

// We need to define a wrapper type in order to retrieve the URL
// in the `subscribe` handler.
// Retrieval from the context, in actix-web, is type-based: using
// a raw `String` would expose us to conflicts.
#[derive(Clone, Debug)]
pub struct ApplicationBaseUrl(pub String);

pub async fn run(
    listener: TcpListener,
    conn_pool: PgPool,
    // email_client: EmailClient,
    base_url: String,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    tracing::debug!("listening on {}", listener.local_addr()?);

    let state = AppState {
        database: conn_pool,
        base_url: ApplicationBaseUrl(base_url),
    };

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/greet/:name", get(greet))
        .route("/health_check", get(health_check))
        // .route("/subscriptions", post(subscribe))
        // .route("/subscriptions/confirm", get(confirm))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(state);

    // run it with hyper on localhost:3000
    Ok(axum::Server::from_tcp(listener)?.serve(app.into_make_service()))
}
