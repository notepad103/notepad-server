//! 用户相关路由注册

use axum::routing::{get, post, put};
use axum::Router;

use crate::handlers;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    // 不要用 `nest("/users", …)`：只要内层有 `/:id`，`POST /users/verify` 会先被前缀 `/users` 吃进去，
    // 剩余路径落到 `/:id`（仅 GET）→ 405。这里全部用扁平路径，且静态段在 `:id` 之前。
    Router::new()
        .route("/users/login", post(handlers::login))
        .route("/users/verify", post(handlers::send_verification_code))
        .route("/users/password", put(handlers::update_password))
        .route("/users", post(handlers::create_user))
        .route("/users/:id", get(handlers::get_user))
}
