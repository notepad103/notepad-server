mod authenticated;
mod health;
mod users;

use axum::Router;

use crate::state::AppState;
pub(crate) use authenticated::with_auth;

/// 聚合所有路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(health::router())
        .merge(users::router())
}
