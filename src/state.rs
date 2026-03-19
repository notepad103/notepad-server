//! 应用共享状态（数据库连接池等）

use sqlx::PgPool;

/// 注入到 axum Router 的全局状态
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
