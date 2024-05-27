use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::rebase::query_all as rebase_query_all;
use crate::routes::rebase::query_all_author as rebase_query_all_author;
use crate::routes::rebase::query_by_id as rebase_query_by_id;
use crate::routes::rebase::query_by_tag as rebase_query_by_tag;
use crate::routes::rebase::query_by_time as rebase_query_by_time;
use crate::routes::rebase::query_latest_news as rebase_query_latest_news;
use crate::routes::rebase::query_latest_news_id as rebase_query_latest_news_id;

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer};
use aion_parse::rebase::get_total_rebase_daily_episode;
use aion_types::rebase::rebase_daily::RebaseDaliy;
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use std::time::Duration;
use tokio::time;
use tracing_actix_web::TracingLogger;
use uuid::Uuid;

use crate::routes::health_check;
use crate::routes::index;

pub struct Application {
    pub port: u16,
    pub server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self> {
        let connection_pool = get_connection_pool(&configuration.database);
        let address = configuration.application.connection_string();
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let server = run(listener, connection_pool).await?;

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

pub async fn run(listener: TcpListener, conn_pool: PgPool) -> Result<Server> {
    tracing::info!("listening on {}", listener.local_addr()?);
    let state = web::Data::new(conn_pool);
    let state_clone = state.clone();
    let state1 = state.clone();
    let server =
        HttpServer::new(move || {
            let cors = Cors::default()
                //.allowed_origin("http://localhost:8080") // USE THIS FOR LOCAL DEV
                .allowed_origin("https://all-in-one-news-frontend-davirain-su.vercel.app/")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600);
            App::new()
                .wrap(TracingLogger::default())
                .wrap(cors)
                .route("/", web::get().to(index))
                .route("/health_check", web::get().to(health_check))
                .route(
                    "/rebase/list",
                    web::get().to(rebase_query_all::list_all_items),
                )
                .route(
                    "/rebase/list_all",
                    web::get().to(rebase_query_all::list_all),
                )
                .route(
                    "/rebase/authors",
                    web::get().to(rebase_query_all_author::list_authors),
                )
                .route(
                    "/rebase/tags",
                    web::get().to(rebase_query_by_tag::list_tags),
                )
                .route(
                    "/rebase/time",
                    web::get().to(rebase_query_by_time::list_by_time),
                ) // todo (query have problem)
                .route(
                    "/rebase/latest",
                    web::get().to(rebase_query_latest_news::list_latest_news),
                )
                .route(
                    "/rebase/by_id",
                    web::get().to(rebase_query_by_id::list_by_id),
                )
                .route(
                    "/rebase/ids",
                    web::get().to(rebase_query_latest_news_id::list_latest_news_ids),
                )
                .service(web::resource("/echo").route(
                    web::post().to(|data: String| async move { HttpResponse::Ok().body(data) }),
                ))
                .app_data(state_clone.clone())
        })
        .listen(listener)?
        .run();

    // 使用tokio::spawn启动一个异步任务执行定时操作
    tokio::spawn(async move {
        // 定时执行任务，每天执行一次
        let mut interval = time::interval(Duration::from_secs(60 * 60));

        loop {
            interval.tick().await;

            // 在这里调用您的定时任务函数
            if let Err(err) = process_load_all_rebase_daily(state1.clone()).await {
                eprintln!("process_load_all_rebase_daily 定时任务执行出错: {:?}", err);
            }
        }
    });

    // run it with hyper on localhost:3000
    Ok(server)
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

#[allow(dead_code)]
async fn truncate_rebase_table(pool: &PgPool) -> anyhow::Result<()> {
    // SQL 清空表格内容的语句
    let truncate_table_query = "TRUNCATE TABLE new_rebase_daily";

    // 执行清空表格内容的查询
    sqlx::query(truncate_table_query).execute(pool).await?;

    Ok(())
}

async fn task_rebase_handler(
    rebase_daily: RebaseDaliy,
    conn_pool: web::Data<PgPool>,
) -> anyhow::Result<()> {
    let mut connection_pool = conn_pool.acquire().await?;

    println!("task_rebase_handler 定时任务执行中...");

    let key_id = Uuid::new_v4();

    create_rebase_table(&conn_pool).await?;

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

#[tracing::instrument(name = "process_load_all_rebase_daily", skip(conn_pool))]
pub async fn process_load_all_rebase_daily(conn_pool: web::Data<PgPool>) -> anyhow::Result<()> {
    let total_rebase_daily_episode = get_total_rebase_daily_episode().await?;
    for item in total_rebase_daily_episode {
        let conn_pool = conn_pool.clone();
        task_rebase_handler(RebaseDaliy::try_from(item)?, conn_pool).await?;
    }

    Ok(())
}
