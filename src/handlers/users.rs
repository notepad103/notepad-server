//! 用户相关 HTTP 处理（只做提取参数与调用 service）

use crate::error::AppError;
use crate::models::CreateUserRequest;
use crate::services;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<crate::models::UserResponse>), AppError> {
    let user = services::create_user_service(&state.db, req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<crate::models::UserResponse>), AppError> {
    println!("id: {:?}", id);
    let user = services::get_user(&state.db, id).await?;
    Ok((StatusCode::OK, Json(user)))
}

pub async fn send_verification_code(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> Result<(StatusCode, Json<()>), AppError> {
    services::send_verification_code(&state.db, state.redis.clone(), &email).await?;
    Ok((StatusCode::OK, Json(())))
}
