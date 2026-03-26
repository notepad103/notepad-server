//! 用户相关业务逻辑（与 HTTP 无关）

use crate::utils::email;
use chrono::Utc;
use rand::Rng;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use sqlx::Row;

use crate::error::AppError;
use crate::models::{CreateUserRequest, UserResponse};
use sqlx::PgPool;

/// 创建用户：校验邮箱验证码（Redis `verify:email:{email}`），成功写入后再删除该键。
pub async fn create_user(
    pool: &PgPool,
    redis: Option<ConnectionManager>,
    req: CreateUserRequest,
) -> Result<UserResponse, AppError> {
    let Some(mut redis_service) = redis else {
        return Err(AppError::BadRequest(
            "验证码服务未启用（请配置 REDIS_URL）".into(),
        ));
    };

    let key = format!("verify:email:{}", req.email);
    let stored: Option<String> = redis_service.get(&key).await?;
    let input = req.verification_code.trim();
    match stored.as_deref() {
        Some(s) if s == input => {}
        Some(_) => {
            return Err(AppError::BadRequest("验证码错误".into()));
        }
        None => {
            return Err(AppError::BadRequest(
                "验证码无效或已过期，请先获取邮箱验证码".into(),
            ));
        }
    }

    let row = sqlx::query(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id, username, email, created_at",
    )
    .bind(&req.username)
    .bind(&req.email)
    .bind(&req.password)
    .fetch_one(pool)
    .await?;

    let _: () = redis_service.del(&key).await?;

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

/// 邮箱 + 密码登录（当前与库中 `password` 字段明文比对，后续可改为密码哈希）
pub async fn login(pool: &PgPool, email: &str, password: &str) -> Result<UserResponse, AppError> {
    let row = sqlx::query(
        "SELECT id, username, email, created_at, password FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::Unauthorized("邮箱或密码错误".into()));
    };

    let stored: Option<String> = row
        .try_get::<Option<String>, _>("password")
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let Some(ref stored) = stored else {
        return Err(AppError::Unauthorized("邮箱或密码错误".into()));
    };
    if stored != password {
        return Err(AppError::Unauthorized("邮箱或密码错误".into()));
    }

    Ok(UserResponse {
        id: row.get(0),
        username: row.get(1),
        email: row.get(2),
        created_at: row.get(3),
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

/// 发送验证码：将验证码写入 Redis（`verify:email:{email}`），默认 300 秒过期，并发送邮件。
///
/// 注册流程：先调本接口收码，再 `POST /users` 携带同一 `verification_code`。
///
/// - `redis` 为 `None`（未配置 `REDIS_URL`）时返回 `BadRequest`。
pub async fn send_verification_code(
    redis: Option<ConnectionManager>,
    email: &str,
) -> Result<(), AppError> {
    let Some(mut redis_service) = redis else {
        return Err(AppError::BadRequest(
            "验证码服务未启用（请配置 REDIS_URL）".into(),
        ));
    };

    let code: u32 = rand::thread_rng().gen_range(100_000..=999_999);
    let key = format!("verify:email:{email}");
    let ttl_secs = 300u64;
    email::smtp_transport::send_verification_code(email, code).await?;
    let _: () = redis_service
        .set_ex(&key, code.to_string(), ttl_secs)
        .await?;

    Ok(())
}
