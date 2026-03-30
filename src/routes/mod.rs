mod authenticated;
mod health;
mod users;

use axum::Router;

use crate::state::AppState;
/// 聚合所有路由（默认鉴权，公开路径见 `authenticated::is_public_route`）
pub fn routes() -> Router<AppState> {
    authenticated::with_default_auth(
        Router::new()
            .merge(health::router())
            .merge(users::router()),
    )
}
