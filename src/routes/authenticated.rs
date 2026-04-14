//! 默认全路由鉴权：`Authorization: Bearer` JWT；
//! 白名单内的路径不做校验（登录、注册、健康检查等）

use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use axum::http::Method;
use axum::middleware::Next;
use axum::response::Response;
use axum::Router;
use std::collections::HashMap;

use crate::auth::verify_token;
use crate::error::AppError;
use crate::state::AppState;

/// 给整棵路由树加默认鉴权（内部按 [`is_public_route`] 放行）
///
/// 使用 [`Router::layer`] 而不是 [`Router::route_layer`]：
/// `route_layer` 往往只在「已经匹配到某条路由」之后才跑；未匹配（404）或某些边界情况下中间件不会执行，
/// 调试时就会误以为「没打到中间件」。
pub fn with_default_auth(router: Router<AppState>) -> Router<AppState> {
    router.layer(axum::middleware::from_fn(require_auth_unless_public))
}

pub fn is_public_route(method: &Method, path: &str) -> bool {
    let method_map = HashMap::from([
        (Method::POST, vec!["/users", "/users/login", "/users/verify"]),
        (Method::PUT, vec!["/users/password"]),
    ]);
    method_map.get(method).unwrap_or(&vec![]).contains(&path)
}

pub async fn require_auth_unless_public(
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let method = req.method().clone();
    let path = req.uri().path().to_owned();
    if is_public_route(&method, &path) {
        return Ok(next.run(req).await);
    }

    let hdr = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or_else(|| AppError::Unauthorized("缺少 Authorization 头".into()))?
        .to_str()
        .map_err(|_| AppError::Unauthorized("Authorization 头无效".into()))?;
    let token = hdr
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("须使用 Bearer 令牌".into()))?;
    verify_token(token)?;
    Ok(next.run(req).await)
}
