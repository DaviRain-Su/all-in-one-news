use aion_types::rebase::rebase_daily::RebaseDaliy;
use all_in_one_news::configuration::get_configuration;
use all_in_one_news::startup::get_connection_pool;
use all_in_one_news::startup::Application;
use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::{task, time};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

async fn task_handler(rebase_daily: RebaseDaliy, conn_pool: Arc<Mutex<PgPool>>) {
    // let pg = pg.lock().unwrap().acquire().await.unwrap();
    let mut connection_pool = conn_pool
        .lock()
        .unwrap()
        .acquire()
        .await
        .expect("Failed to acquire connection");
    println!("定时任务执行中...");
    let key_id = Uuid::new_v4();
    // 在这里编写你的定时任务逻辑
    // 执行插入操作
    let tags = rebase_daily
        .tag
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    let result = sqlx::query!(
        r#"
            INSERT INTO rebase_daily (key_id, id, author, episode, introduce, time, title, url, tag)
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
        Ok(_) => println!("插入成功"),
        Err(e) => println!("插入失败: {:?}", e),
    }
}

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

    let pg_pool = get_connection_pool(&configuration.database);
    let pg = Arc::new(Mutex::new(pg_pool));

    let task = task::spawn(async move {
        loop {
            let pg = pg.clone();
            // task_handler(pg).await;
            time::sleep(Duration::from_secs(60 * 60 * 24)).await; // 每隔24小时执行一次定时任务
        }
    });

    let service = Application::build(configuration).await?;
    println!("🌟🌟🌟🌟🌟🌟 Server is running on port 8000 🌟🌟🌟🌟🌟");

    service.run_until_stopped().await?;

    task.await?;

    Ok(())
}
