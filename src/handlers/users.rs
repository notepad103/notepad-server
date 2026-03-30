//! 用户相关 HTTP 处理（只做提取参数与调用 service）

use crate::auth::{self, AuthUser, Ddd};
use crate::error::AppError;
use crate::models::{CreateUserRequest, LoginRequest, LoginResponse};
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
    let user = services::create_user_service(&state.db, state.redis.clone(), req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), AppError> {
    let user = services::login(&state.db, &req.email, &req.password).await?;
    let access_token = auth::sign_token(user.id)?;
    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            access_token,
            token_type: "Bearer",
            user,
        }),
    ))
}

pub async fn get_user(
    AuthUser(auth_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<crate::models::UserResponse>), AppError> {
    if auth_id != id {
        return Err(AppError::Forbidden("只能查看本人的用户信息".into()));
    }
    let user = services::get_user(&state.db, id).await?;
    Ok((StatusCode::OK, Json(user)))
}

pub async fn send_verification_code(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> Result<(StatusCode, Json<()>), AppError> {
    services::send_verification_code(state.redis.clone(), &email).await?;
    Ok((StatusCode::OK, Json(())))
}
