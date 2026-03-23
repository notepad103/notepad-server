//! 健康检查等轻量逻辑

use redis::aio::ConnectionManager;
use sqlx::PgPool;

/// 探测数据库连接是否正常，返回查询结果
pub async fn db_ping(pool: &PgPool) -> Result<i32, sqlx::Error> {
    let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(pool).await?;
    Ok(row.0)
}

/// Redis `PING`，成功时一般为 `"PONG"`
pub async fn redis_ping(manager: &ConnectionManager) -> Result<String, redis::RedisError> {
    let mut conn = manager.clone();
    redis::cmd("PING").query_async::<String>(&mut conn).await
}
