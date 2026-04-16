//! 统一 API 错误类型与 HTTP 响应

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::error;

/// 应用层错误，可映射为 HTTP 状态码与 JSON body
#[derive(Debug)]
pub enum AppError {
    /// 数据库等内部错误
    Internal(String),
    /// 冲突（如唯一约束）
    Conflict(String),
    /// 客户端请求错误
    BadRequest(String),
    /// 未认证或令牌无效
    Unauthorized(String),
    /// 已认证但无权限
    Forbidden(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
        }
    }

    fn message(&self) -> &str {
        match self {
            AppError::Internal(s) => s.as_str(),
            AppError::Conflict(s) => s.as_str(),
            AppError::BadRequest(s) => s.as_str(),
            AppError::Unauthorized(s) => s.as_str(),
            AppError::Forbidden(s) => s.as_str(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        if status == StatusCode::INTERNAL_SERVER_ERROR {
            error!(error = self.message(), "internal server error");
        }
        let body = Json(json!({ "code": status.as_u16(), "error": self.message() }));
        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        let msg = e.to_string();
        if msg.contains("unique") || msg.contains("duplicate") {
            AppError::Conflict(msg)
        } else {
            AppError::Internal(msg)
        }
    }
}

impl From<redis::RedisError> for AppError {
    fn from(e: redis::RedisError) -> Self {
        AppError::Internal(e.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        AppError::Unauthorized("无效或过期的令牌".into())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Internal(e.to_string())
    }
}
