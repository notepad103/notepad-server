use crate::{config::Config, models::UserResponse};
use redis::{AsyncCommands, aio::ConnectionManager};
use sqlx::{Row, postgres::PgPoolOptions};

// pub async fn get_user_cache(user_id: i32) -> Result<UserResponse, AppError> {
//     let config = Config::global();
//     let redis = match config.redis_url.as_deref() {
//         Some(url) => {
//             let client = redis::Client::open(url)?;
//             Some(ConnectionManager::new(client).await?)
//         }
//         None => None,
//     };
//     let Some(mut redis_service) = redis else {
//         return Err(AppError::BadRequest(
//             "验证码服务未启用（请配置 REDIS_URL）".into(),
//         ));
//     };
//     let key = format!("user:login:{}", user_id);
//     let user_cache_value: Option<String> = redis_service.get(&key).await?;
//     let raw = user_cache_value.ok_or_else(|| {
//         AppError::Unauthorized(format!("登录缓存不存在或已过期: {}", user_id))
//     })?;
//     let user: UserResponse = serde_json::from_str(&raw)
//         .map_err(|e| AppError::Internal(format!("parse user cache failed: {e}")))?;
//     Ok(user)
// }

pub async fn get_user_cache(user_id: i32) -> Option<UserResponse> {
    let config = Config::global();
    let redis = match config.redis_url.as_deref() {
        Some(url) => {
            let client = redis::Client::open(url).ok()?;
            Some(ConnectionManager::new(client).await.ok()?)
        }
        None => None,
    };
    let Some(mut redis_service) = redis else { return None };
    let key = format!("user:login:{}", user_id);
    let user_cache_value: Option<String> = redis_service.get(&key).await.ok()?;
    let raw = user_cache_value?;
    serde_json::from_str::<UserResponse>(&raw).ok()
}

pub async fn get_user_db(user_id: i32) -> Option<UserResponse> {
    let config = Config::global();
    let pg = PgPoolOptions::new()
        .max_connections(1)
        .connect(&config.database_url)
        .await
        .ok()?;
    let row = sqlx::query(
        "SELECT id, username, email, created_at FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_one(&pg)
    .await
    .ok()?;
    // 如果没有查询到信息，则提前返回 None
    if row.is_empty() {
        return None;
    }
    
    let user_info = UserResponse {
        id: row.get(0),
        username: row.get(1),
        email: row.get(2),
        created_at: row.get(3),
    };
    Some(user_info)
}

pub async fn get_user(user_id: i32) -> Option<UserResponse> {
    if let Some(user_info) = get_user_cache(user_id).await {
        Some(user_info)
    } else {
        get_user_db(user_id).await
    }
}
