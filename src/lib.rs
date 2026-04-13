//! notepad 应用库入口，供 main 使用

mod auth;
mod config;
mod error;
mod handlers;
mod models;
mod routes;
mod services;
mod state;
mod utils;
mod other;

pub use config::Config;
pub use error::AppError;
pub use state::AppState;

use redis::aio::ConnectionManager;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// 初始化 tracing-subscriber，从 RUST_LOG 环境变量读取过滤级别。
/// 若 RUST_LOG 未设置，默认只打印本 crate 的 INFO 及以上日志。
pub fn init_tracing() {
    let default_filter = format!("{}=info,tower_http=info", env!("CARGO_PKG_NAME"));
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| default_filter.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

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
    let app = routes::routes()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    info!("HTTP 服务启动: http://{}", config.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
