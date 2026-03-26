//! JWT（HS256）签发、校验与 `Authorization: Bearer` 提取器

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::AppError;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

/// 签发访问令牌，`sub` 为用户 id 字符串
pub fn sign_token(user_id: i32) -> Result<String, AppError> {
    let c = Config::global();
    let exp = Utc::now().timestamp() + c.jwt_exp_secs as i64;
    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(c.jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::Internal("签发令牌失败".into()))
}

pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    let c = Config::global();
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(c.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(token.claims)
}

/// 从请求头解析 Bearer JWT，得到已认证用户 id
pub struct AuthUser(pub i32);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let hdr = match parts.headers.get(AUTHORIZATION) {
            None => return Err(AppError::Unauthorized("缺少 Authorization 头".into())),
            Some(v) => v
                .to_str()
                .map_err(|_| AppError::Unauthorized("Authorization 头无效".into()))?,
        };
        let token = hdr
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("须使用 Bearer 令牌".into()))?;
        let claims = verify_token(token)?;
        let id: i32 = claims
            .sub
            .parse()
            .map_err(|_| AppError::Unauthorized("令牌主体无效".into()))?;
        Ok(AuthUser(id))
    }
}
