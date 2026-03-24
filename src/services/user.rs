//! 用户相关业务逻辑（与 HTTP 无关）

use chrono::Utc;
use rand::Rng;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use sqlx::Row;
use crate::utils::email;

use crate::error::AppError;
use crate::models::{CreateUserRequest, UserResponse};
use sqlx::PgPool;

/// 创建用户，返回插入后的用户信息
pub async fn create_user(pool: &PgPool, req: CreateUserRequest) -> Result<UserResponse, AppError> {
    let row = sqlx::query(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id, username, email, created_at",
    )
    .bind(&req.username)
    .bind(&req.email)
    .bind(&req.password)
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

pub async fn get_user(pool: &PgPool, id: i32) -> Result<UserResponse, AppError> {
    let row = sqlx::query("SELECT id, username, email, created_at FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(UserResponse {
        id: row.get(0),
        username: row.get(1),
        email: row.get(2),
        created_at: row.get(3),
    })
}

/// 发送验证码：校验用户存在后，将验证码写入 Redis（`verify:email:{email}`），默认 300 秒过期。
///
/// - `redis` 为 `None`（未配置 `REDIS_URL`）时返回 `BadRequest`，避免误以为已发码。
/// - 调用方传入 `state.redis.clone()` 即可（`ConnectionManager` 克隆成本低）。
#[allow(dead_code)]
pub async fn send_verification_code(
    pool: &PgPool,
    redis: Option<ConnectionManager>,
    email: &str,
) -> Result<(), AppError> {
    sqlx::query_scalar::<_, i32>("SELECT id FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await?;

    let Some(mut redis_service) = redis else {
        return Err(AppError::BadRequest(
            "验证码服务未启用（请配置 REDIS_URL）".into(),
        ));
    };
    

    // 1000..2000.gen_range()
    let code: u32 = rand::thread_rng().gen_range(100_000..=999_999);
    let key = format!("verify:email:{email}");
    let ttl_secs = 300u64;
    //  email::smtp_transport::send_verification_code(&mut email_service, email, code).await?;
    email::smtp_transport::send_verification_code(email, code).await?;
    let _: () = redis_service
        .set_ex(&key, code.to_string(), ttl_secs)
        .await?;

    Ok(())
}
