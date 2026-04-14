//! 用户相关 HTTP 处理（只做提取参数与调用 service）

use crate::auth::{self, AuthUser};
use crate::error::AppError;
use crate::models::{
    CreateUserRequest, LoginRequest, LoginResponse, SendVerificationCodeRequest,
    UpdatePasswordRequest,
};
use crate::response::ApiResponse;
use crate::services;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<ApiResponse<crate::models::UserResponse>, AppError> {
    let user = services::create_user_service(&state.db, state.redis.clone(), req).await?;
    Ok(ApiResponse(user))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<ApiResponse<LoginResponse>, AppError> {
    let user = services::login(&state.db, state.redis.clone(), &req.email, &req.password).await?;
    let access_token = auth::sign_token(user.id)?;
    Ok(ApiResponse(LoginResponse {
        access_token,
        token_type: "Bearer",
        user,
    }))
}

pub async fn get_user(
    AuthUser(auth_id): AuthUser,
    Path(id): Path<i32>,
) -> Result<ApiResponse<crate::models::UserResponse>, AppError> {
    if auth_id != id {
        return Err(AppError::Forbidden("只能查看本人的用户信息".into()));
    }
    let user = services::get_user(id).await?;
    Ok(ApiResponse(user))
}

pub async fn send_verification_code(
    State(state): State<AppState>,
    Json(req): Json<SendVerificationCodeRequest>,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    services::send_verification_code(state.redis.clone(), &req.email).await?;
    Ok(ApiResponse(serde_json::json!({ "message": "验证码发送成功" })))
}

pub async fn update_password(
    State(state): State<AppState>,
    Json(req): Json<UpdatePasswordRequest>,
) -> Result<ApiResponse<()>, AppError> {
    services::update_password(&state.db, &req.email, &req.password, &req.new_password).await?;
    Ok(ApiResponse(()))
}
