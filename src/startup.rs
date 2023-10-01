use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::rebase::query_all as rebase_query_all;
use crate::routes::rebase::query_all_author as rebase_query_all_author;
use crate::routes::rebase::query_by_id as rebase_query_by_id;
use crate::routes::rebase::query_by_tag as rebase_query_by_tag;
use crate::routes::rebase::query_by_time as rebase_query_by_time;
use crate::routes::rebase::query_latest_news as rebase_query_latest_news;
use crate::routes::rebase::query_latest_news_id as rebase_query_latest_news_id;

use crate::routes::rustcc::query_all as rustcc_query_all;
use crate::routes::rustcc::query_all_author as rustcc_query_all_author;
use crate::routes::rustcc::query_by_id as rustcc_query_by_id;
use crate::routes::rustcc::query_by_tag as rustcc_query_by_tag;
use crate::routes::rustcc::query_by_time as rustcc_query_by_time;
use crate::routes::rustcc::query_latest_news as rustcc_query_latest_news;
use crate::routes::rustcc::query_latest_news_id as rustcc_query_latest_news_id;

use aion_parse::rebase::get_total_rebase_daily_episode;
use aion_parse::rustcc::get_total_rustcc_daily_episode;
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
        .route("/rebase/list", get(rebase_query_all::list_all_items))
        .route("/rebase/list_all", get(rebase_query_all::list_all))
        .route(
            "/rebase/authors",
            get(rebase_query_all_author::list_authors),
        )
        .route("/rebase/tags", get(rebase_query_by_tag::list_tags))
        .route("/rebase/time", get(rebase_query_by_time::list_by_time)) // todo (query have problem)
        .route(
            "/rebase/latest",
            get(rebase_query_latest_news::list_latest_news),
        )
        .route("/rebase/by_id", get(rebase_query_by_id::list_by_id))
        .route(
            "/rebase/ids",
            get(rebase_query_latest_news_id::list_latest_news_ids),
        )
        .route("/rustcc/list", get(rustcc_query_all::list_all_items))
        .route("/rustcc/list_all", get(rustcc_query_all::list_all))
        .route(
            "/rustcc/authors",
            get(rustcc_query_all_author::list_authors),
        )
        .route("/rustcc/tags", get(rustcc_query_by_tag::list_tags))
        .route("/rustcc/time", get(rustcc_query_by_time::list_by_time)) // todo (query have problem)
        .route(
            "/rustcc/latest",
            get(rustcc_query_latest_news::list_latest_news),
        )
        .route("/rustcc/by_id", get(rustcc_query_by_id::list_by_id))
        .route(
            "/rustcc/ids",
            get(rustcc_query_latest_news_id::list_latest_news_ids),
        )
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
                .allow_origin(HeaderValue::from_static("*"))
                .allow_methods([Method::GET]),
        )
        .with_state(state);

    let pg_pool = conn_pool;
    let pg = Arc::new(pg_pool);
    let pg_clone = pg.clone();
    let pg_clone1 = pg.clone();

    // 使用tokio::spawn启动一个异步任务执行定时操作
    tokio::spawn(async move {
        // 定时执行任务，每天执行一次
        let mut interval = time::interval(Duration::from_secs(60 * 60 * 24));

        loop {
            interval.tick().await;

            // 在这里调用您的定时任务函数
            if let Err(err) = process_load_all_rebase_daily(pg_clone.clone()).await {
                eprintln!("process_load_all_rebase_daily 定时任务执行出错: {:?}", err);
            }
        }
    });

    // 使用tokio::spawn启动一个异步任务执行定时操作
    tokio::spawn(async move {
        // 定时执行任务，每天执行一次
        let mut interval = time::interval(Duration::from_secs(60 * 60 * 24));

        loop {
            interval.tick().await;

            // 在这里调用您的定时任务函数
            if let Err(err) = process_load_all_rustcc_daily(pg_clone1.clone()).await {
                eprintln!("task_rustcc_handler 定时任务执行出错: {:?}", err);
            }
        }
    });

    // run it with hyper on localhost:3000
    Ok(axum::Server::from_tcp(listener)?.serve(app.into_make_service()))
}

async fn create_rebase_table(pool: &PgPool) -> anyhow::Result<()> {
    // SQL 创建表格的语句
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS new_rebase_daily (
            key_id UUID PRIMARY KEY NOT NULL,
            hash TEXT NOT NULL,
            id INTEGER NOT NULL,
            author TEXT NOT NULL,
            episode TEXT NOT NULL,
            introduce TEXT NOT NULL,
            time TIMESTAMPTZ NOT NULL,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            tag TEXT[] NOT NULL
        )
    "#;

    // 执行创建表格的查询
    sqlx::query(create_table_query).execute(pool).await?;

    Ok(())
}

async fn truncate_rebase_table(pool: &PgPool) -> anyhow::Result<()> {
    // SQL 清空表格内容的语句
    let truncate_table_query = "TRUNCATE TABLE new_rebase_daily";

    // 执行清空表格内容的查询
    sqlx::query(truncate_table_query).execute(pool).await?;

    Ok(())
}

async fn create_rustcc_table(pool: &PgPool) -> anyhow::Result<()> {
    // SQL 创建表格的语句
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS new_rustcc_daily (
            key_id UUID PRIMARY KEY NOT NULL,
            hash TEXT NOT NULL,
            id INTEGER NOT NULL,
            author TEXT NOT NULL,
            episode TEXT NOT NULL,
            introduce TEXT NOT NULL,
            time TIMESTAMPTZ NOT NULL,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            tag TEXT[] NOT NULL
        )
    "#;

    // 执行创建表格的查询
    sqlx::query(create_table_query).execute(pool).await?;

    Ok(())
}

async fn task_rebase_handler(
    rebase_daily: RebaseDaliy,
    conn_pool: Arc<PgPool>,
) -> anyhow::Result<()> {
    let mut connection_pool = conn_pool.acquire().await?;

    println!("task_rebase_handler 定时任务执行中...");

    let key_id = Uuid::new_v4();

    create_rebase_table(&conn_pool).await?;
    truncate_rebase_table(&conn_pool).await?;

    // 在这里编写你的定时任务逻辑
    // 执行插入操作
    // 检查是否已存在相同 HASH 的记录
    let existing_record = sqlx::query!(
        "SELECT id FROM new_rebase_daily WHERE hash = $1",
        rebase_daily.hash
    )
    .fetch_optional(connection_pool.as_mut())
    .await;

    match existing_record {
        Ok(Some(_)) => {
            println!("task_rebase_handler 相同 ID 的记录已存在，不执行插入操作");
        }
        Ok(None) => {
            // 如果不存在相同 ID 的记录，则执行插入操作
            // TODO( tag 可以在后面更新)
            let tags = rebase_daily
                .tag
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>();

            let result = sqlx::query!(
                           r#"
                           INSERT INTO new_rebase_daily (key_id, hash, id, author, episode, introduce, time, title, url, tag)
                           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                           "#,
                           key_id,
                           rebase_daily.hash,
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
                    println!("task_rebase_handler 插入成功");
                }
                Err(e) => {
                    println!("task_rebase_handler 插入失败: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("task_rebase_handler 检查记录时出错: {:?}", e);
        }
    }
    Ok(())
}

async fn task_rustcc_handler(
    rebase_daily: RebaseDaliy,
    conn_pool: Arc<PgPool>,
) -> anyhow::Result<()> {
    let mut connection_pool = conn_pool.acquire().await?;

    println!("task_rustcc_handler 定时任务执行中...");

    let key_id = Uuid::new_v4();

    create_rustcc_table(&conn_pool).await?;

    // 在这里编写你的定时任务逻辑
    // 执行插入操作
    // 检查是否已存在相同 HASH 的记录
    let existing_record = sqlx::query!(
        "SELECT id FROM new_rustcc_daily WHERE hash = $1",
        rebase_daily.hash
    )
    .fetch_optional(connection_pool.as_mut())
    .await;

    match existing_record {
        Ok(Some(_)) => {
            println!("task_rustcc_handler 相同 ID 的记录已存在，不执行插入操作");
        }
        Ok(None) => {
            // 如果不存在相同 ID 的记录，则执行插入操作
            // TODO( tag 可以在后面更新)
            let tags = rebase_daily
                .tag
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>();

            let result = sqlx::query!(
                           r#"
                           INSERT INTO new_rustcc_daily (key_id, hash, id, author, episode, introduce, time, title, url, tag)
                           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                           "#,
                           key_id,
                           rebase_daily.hash,
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
                    println!("task_rustcc_handler 插入成功");
                }
                Err(e) => {
                    println!("task_rustcc_handler 插入失败: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("task_rustcc_handler 检查记录时出错: {:?}", e);
        }
    }
    Ok(())
}

pub async fn process_load_all_rebase_daily(conn_pool: Arc<PgPool>) -> anyhow::Result<()> {
    let total_rebase_daily_episode = get_total_rebase_daily_episode().await?;
    for item in total_rebase_daily_episode {
        let conn_pool = conn_pool.clone();
        task_rebase_handler(RebaseDaliy::try_from(item)?, conn_pool).await?;
    }

    Ok(())
}

pub async fn process_load_all_rustcc_daily(conn_pool: Arc<PgPool>) -> anyhow::Result<()> {
    let total_rustcc_daily_episode = get_total_rustcc_daily_episode().await?;
    for item in total_rustcc_daily_episode {
        let conn_pool = conn_pool.clone();
        task_rustcc_handler(RebaseDaliy::try_from(item)?, conn_pool).await?;
    }

    Ok(())
}
