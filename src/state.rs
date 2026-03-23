//! 应用共享状态（数据库连接池等）

use redis::aio::ConnectionManager;
use sqlx::PgPool;

/// 注入到 axum Router 的全局状态
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    /// 未配置 `REDIS_URL` 时为 `None`
    pub redis: Option<ConnectionManager>,
}

impl AppState {
    pub fn new(db: PgPool, redis: Option<ConnectionManager>) -> Self {
        Self { db, redis }
    }
}
