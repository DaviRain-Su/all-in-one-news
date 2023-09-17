use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::query_all::{list_all, list_all_items};
use crate::routes::query_all_author::list_authors;
use crate::routes::query_by_id::list_by_id;
use crate::routes::query_by_tag::list_tags;
use crate::routes::query_by_time::list_by_time;
use crate::routes::query_latest_news::list_latest_news;
use crate::routes::query_latest_news_id::list_latest_news_ids;
use aion_parse::rebase::get_total_rebase_daily_episode;
use aion_types::parse_key::parse_tag;
use aion_types::rebase::rebase_daily::RebaseDaliy;
use anyhow::Result;
use axum::http::{HeaderValue, Method};
use axum::routing::IntoMakeService;
use axum::Server;
use axum::{routing::get, Router};
use hyper::server::conn::AddrIncoming;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use uuid::Uuid;

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
    base_url: String,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>> {
    tracing::debug!("listening on {}", listener.local_addr()?);

    let state = AppState {
        database: conn_pool.clone(),
        base_url: ApplicationBaseUrl(base_url),
    };

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/health_check", get(health_check))
        .route("/list", get(list_all_items))
        .route("/list_all", get(list_all))
        .route("/authors", get(list_authors))
        .route("/tags", get(list_tags))
        .route("/time", get(list_by_time)) // todo (query have problem)
        .route("/latest", get(list_latest_news))
        .route("/by_id", get(list_by_id))
        .route("/ids", get(list_latest_news_ids))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
        .with_state(state);

    let pg_pool = conn_pool;
    let pg = Arc::new(pg_pool);
    let pg_clone = pg.clone();

    // 使用tokio::spawn启动一个异步任务执行定时操作
    tokio::spawn(async move {
        // 定时执行任务，例如每小时执行一次
        let mut interval = time::interval(Duration::from_secs(60 * 60));

        loop {
            interval.tick().await;

            // 在这里调用您的定时任务函数
            if let Err(err) = process_load_all_rebase_daily(pg_clone.clone()).await {
                eprintln!("定时任务执行出错: {:?}", err);
            }
        }
    });

    // run it with hyper on localhost:3000
    Ok(axum::Server::from_tcp(listener)?.serve(app.into_make_service()))
}

async fn task_handler(rebase_daily: RebaseDaliy, conn_pool: Arc<PgPool>) -> anyhow::Result<()> {
    let mut connection_pool = conn_pool.acquire().await?;

    println!("定时任务执行中...");

    let key_id = Uuid::new_v4();
    // 在这里编写你的定时任务逻辑
    // 执行插入操作
    // 检查是否已存在相同 ID 的记录
    let existing_record = sqlx::query!(
        "SELECT id FROM new_rebase_daily WHERE id = $1",
        rebase_daily.id as i32
    )
    .fetch_optional(connection_pool.as_mut())
    .await;

    match existing_record {
        Ok(Some(_)) => {
            println!("相同 ID 的记录已存在，不执行插入操作");
            Err(anyhow::anyhow!("相同 ID 的记录已存在，不执行插入操作"))
        }
        Ok(None) => {
            // 如果不存在相同 ID 的记录，则执行插入操作
            let mut result = parse_tag(&rebase_daily.introduce, 3).await?;
            let mut tags = rebase_daily.tag;
            tags.append(&mut result);
            let tags = tags
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>();

            let result = sqlx::query!(
                           r#"
                           INSERT INTO new_rebase_daily (key_id, id, author, episode, introduce, time, title, url, tag)
                           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                           "#,
                           key_id,
                           rebase_daily.id as i32,
                           rebase_daily.author,
                           rebase_daily.episode,
                           rebase_daily.introduce,
                           rebase_daily.time,
                           rebase_daily.title,
                           rebase_daily.url,
                           &tags // 注意此处使用引用来插入 Vec<String>
                       )
                       .execute(connection_pool.as_mut())
                       .await;

            match result {
                Ok(_) => {
                    println!("插入成功");
                    Ok(())
                }
                Err(e) => {
                    println!("插入失败: {:?}", e);
                    Err(anyhow::anyhow!("插入失败: {:?}", e))
                }
            }
        }
        Err(e) => {
            println!("检查记录时出错: {:?}", e);
            Err(anyhow::anyhow!("检查记录时出错: {:?}", e))
        }
    }
}

pub async fn process_load_all_rebase_daily(conn_pool: Arc<PgPool>) -> anyhow::Result<()> {
    let total_rebase_daily_episode = get_total_rebase_daily_episode().await?;
    for item in total_rebase_daily_episode {
        let conn_pool = conn_pool.clone();
        task_handler(RebaseDaliy::try_from(item)?, conn_pool).await?;
    }

    Ok(())
}
