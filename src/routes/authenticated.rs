//! 可复用的鉴权路由包装，供各模块统一套用

use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use axum::{Router, middleware};

use crate::auth::verify_token;
use crate::error::AppError;
use crate::state::AppState;

/// 给一组路由添加统一鉴权（Bearer JWT）
pub fn with_auth(router: Router<AppState>) -> Router<AppState> {
    router.route_layer(middleware::from_fn(require_auth))
}

async fn require_auth(req: Request, next: Next) -> Result<Response, AppError> {
    let hdr = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or_else(|| AppError::Unauthorized("缺少 Authorization 头".into()))?
        .to_str()
        .map_err(|_| AppError::Unauthorized("Authorization 头无效".into()))?;
    let token = hdr
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("须使用 Bearer 令牌".into()))?;
    let _claims = verify_token(token)?;
    Ok(next.run(req).await)
}
