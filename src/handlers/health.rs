//! 健康与根路径的 HTTP 处理

use axum::{extract::State, Json};
use serde_json::Value;

use crate::services;
use crate::state::AppState;

pub async fn root(State(state): State<AppState>) -> Json<Value> {
    let db = services::db_ping(&state.db).await.unwrap_or(0);
    Json(serde_json::json!({ "ok": true, "db": db }))
}

pub async fn health() -> &'static str {
    "ok"
}
