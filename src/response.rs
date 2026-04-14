//! 统一成功响应包装中间件：将 2xx 响应体包装为 `{ "code": 200, "data": <原始数据> }`
//! 4xx/5xx 由 AppError 自行处理，直接放行。

use axum::{
    body::Body,
    http::{Response, StatusCode, header},
};

pub async fn wrap_response(response: Response<Body>) -> Response<Body> {
    if !response.status().is_success() {
        return response;
    }

    let (mut parts, body) = response.into_parts();

    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .unwrap_or_default();

    let data: serde_json::Value = if bytes.is_empty() {
        serde_json::Value::Null
    } else if let Ok(v) = serde_json::from_slice(&bytes) {
        v
    } else {
        serde_json::Value::String(String::from_utf8_lossy(&bytes).into_owned())
    };

    let new_bytes =
        serde_json::to_vec(&serde_json::json!({ "code": 200, "data": data })).unwrap_or_default();

    parts.status = StatusCode::OK;
    parts.headers.insert(
        header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );
    parts.headers.remove(header::CONTENT_LENGTH);

    Response::from_parts(parts, Body::from(new_bytes))
}
