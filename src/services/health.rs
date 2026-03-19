//! 健康检查等轻量逻辑

use sqlx::PgPool;

/// 探测数据库连接是否正常，返回查询结果
pub async fn db_ping(pool: &PgPool) -> Result<i32, sqlx::Error> {
    let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(pool).await?;
    Ok(row.0)
}
