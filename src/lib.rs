//! notepad 应用库入口，供 main 与测试使用

mod config;
mod error;
mod handlers;
mod models;
mod routes;
mod services;
mod state;

pub use config::Config;
pub use error::AppError;
pub use state::AppState;

use redis::aio::ConnectionManager;
use sqlx::postgres::PgPoolOptions;

/// 构建并运行 HTTP 服务（使用全局 Config，需先调用 Config::init_global()）
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::global();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate::Migrator::new(std::path::Path::new("migrations"))
        .await?
        .run(&pool)
        .await?;

    let redis = match config.redis_url.as_deref() {
        Some(url) => {
            let client = redis::Client::open(url)?;
            Some(ConnectionManager::new(client).await?)
        }
        None => None,
    };

    let state = AppState::new(pool, redis);
    let app = routes::routes().with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    println!("HTTP 服务: http://{}", config.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
