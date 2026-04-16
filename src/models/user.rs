//! 用户相关 DTO

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub user: UserResponse,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    /// 与 `POST /users/verify` 下发到 Redis 的验证码一致
    pub verification_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl fmt::Display for UserResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UserResponse(id: {}, username: {}, email: {}, created_at: {})", self.id, self.username, self.email, self.created_at)
    }
}

#[derive(Debug, Deserialize)]
pub struct SendVerificationCodeRequest {
    pub email: String,
}


#[derive(Debug, Deserialize)]
pub struct UpdatePasswordRequest {
    pub email: String,
    pub password: String,
    pub new_password: String,
}

