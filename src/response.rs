//! 统一成功响应包装：`{ "code": 200, "data": <T> }`

use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

pub struct ApiResponse<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        Json(json!({ "code": 200, "data": self.0 })).into_response()
    }
}
