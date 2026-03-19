//! 用户相关业务逻辑（与 HTTP 无关）

use chrono::Utc;
use sqlx::Row;

use crate::error::AppError;
use crate::models::{CreateUserRequest, UserResponse};
use sqlx::PgPool;

/// 创建用户，返回插入后的用户信息
pub async fn create_user(pool: &PgPool, req: CreateUserRequest) -> Result<UserResponse, AppError> {
    let row = sqlx::query(
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email, created_at",
    )
    .bind(&req.username)
    .bind(&req.email)
    .fetch_one(pool)
    .await?;

    let id: i32 = row.get(0);
    let username: String = row.get(1);
    let email: String = row.get(2);
    let created_at: chrono::DateTime<Utc> = row.get(3);

    Ok(UserResponse {
        id,
        username,
        email,
        created_at,
    })
}
