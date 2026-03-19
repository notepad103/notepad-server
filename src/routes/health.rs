//! 健康检查与根路由注册

use axum::{routing::get, Router};

use crate::handlers;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health))
}
