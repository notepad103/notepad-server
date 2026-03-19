//! 用户相关路由注册

use axum::{routing::post, Router};

use crate::handlers;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/users", post(handlers::create_user))
}
