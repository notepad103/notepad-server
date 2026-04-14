mod authenticated;
mod health;
mod notes;
mod sections;
mod users;

use axum::{Router, middleware::map_response};

use crate::response::wrap_response;
use crate::state::AppState;

/// 聚合所有路由（默认鉴权，公开路径见 `authenticated::is_public_route`）
pub fn routes() -> Router<AppState> {
    authenticated::with_default_auth(
        Router::new()
            // 用户相关路径优先合并，降低与其它子路由在实现细节上的匹配顺序差异
            .merge(users::router())
            .merge(health::router())
            .merge(notes::router())
            .merge(sections::router()),
    )
    .layer(map_response(wrap_response))
}
