//! 用户相关 HTTP 处理（只做提取参数与调用 service）

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use crate::error::AppError;
use crate::models::CreateUserRequest;
use crate::services;
use crate::state::AppState;

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<crate::models::UserResponse>), AppError> {
    let user = services::create_user_service(&state.db, req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}
