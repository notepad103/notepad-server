//! 健康与根路径的 HTTP 处理

use axum::{extract::State, Json};
use serde_json::Value;

use crate::services;
use crate::state::AppState;

pub async fn root(State(state): State<AppState>) -> Json<Value> {
    let db = services::db_ping(&state.db).await.unwrap_or(0);
    let redis = match &state.redis {
        None => serde_json::Value::Null,
        Some(m) => match services::redis_ping(m).await {
            Ok(pong) => serde_json::Value::String(pong),
            Err(_) => serde_json::Value::Bool(false),
        },
    };
    Json(serde_json::json!({ "ok": true, "db": db, "redis": redis }))
}

pub async fn health() -> &'static str {
    "ok"
}

/// 用于确认 3000 端口上跑的是本仓库的 notepad（排查「连错进程 / 旧二进制」）
pub async fn notepad_fingerprint() -> &'static str {
    "notepad-api"
}
