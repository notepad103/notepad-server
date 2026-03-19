//! 统一 API 错误类型与 HTTP 响应

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// 应用层错误，可映射为 HTTP 状态码与 JSON body
#[derive(Debug)]
pub enum AppError {
    /// 数据库等内部错误
    Internal(String),
    /// 冲突（如唯一约束）
    Conflict(String),
    /// 客户端请求错误
    BadRequest(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn message(&self) -> &str {
        match self {
            AppError::Internal(s) => s.as_str(),
            AppError::Conflict(s) => s.as_str(),
            AppError::BadRequest(s) => s.as_str(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(json!({ "error": self.message() }));
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
