//! 用户相关路由注册

use axum::routing::{get, post};
use axum::Router;

use crate::handlers;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/login", post(handlers::login))
        .route("/users", post(handlers::create_user))
        .route("/users/:email/verify", post(handlers::send_verification_code))
        .route("/users/:id", get(handlers::get_user))
}
