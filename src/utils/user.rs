use crate::{AppError, config::Config, models::UserResponse};
use redis::{AsyncCommands, aio::ConnectionManager};

pub async fn get_user_cache(user_id: i32) -> Result<UserResponse, AppError> {
    let config = Config::global();
    let redis = match config.redis_url.as_deref() {
        Some(url) => {
            let client = redis::Client::open(url)?;
            Some(ConnectionManager::new(client).await?)
        }
        None => None,
    };
    let Some(mut redis_service) = redis else {
        return Err(AppError::BadRequest(
            "验证码服务未启用（请配置 REDIS_URL）".into(),
        ));
    };
    let key = format!("user:login:{}", user_id);
    let user_cache_value: Option<String> = redis_service.get(&key).await?;
    let raw = user_cache_value.ok_or_else(|| {
        AppError::Unauthorized(format!("登录缓存不存在或已过期: {}", user_id))
    })?;
    let user: UserResponse = serde_json::from_str(&raw)
        .map_err(|e| AppError::Internal(format!("parse user cache failed: {e}")))?;
    Ok(user)
}
