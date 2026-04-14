use axum::{
    Json,
    extract::{Path, State},
};

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::{CreateSectionRequest, SectionResponse, UpdateSectionRequest};
use crate::response::ApiResponse;
use crate::services;
use crate::state::AppState;

pub async fn list_sections(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
) -> Result<ApiResponse<Vec<SectionResponse>>, AppError> {
    let list = services::list_sections(&state.db, user_id).await?;
    Ok(ApiResponse(list))
}

pub async fn create_section(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(req): Json<CreateSectionRequest>,
) -> Result<ApiResponse<SectionResponse>, AppError> {
    let section = services::create_section(&state.db, user_id, req).await?;
    Ok(ApiResponse(section))
}

pub async fn update_section(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateSectionRequest>,
) -> Result<ApiResponse<SectionResponse>, AppError> {
    let section = services::update_section(&state.db, user_id, &id, req).await?;
    Ok(ApiResponse(section))
}

pub async fn delete_section(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<ApiResponse<()>, AppError> {
    services::delete_section(&state.db, user_id, &id).await?;
    Ok(ApiResponse(()))
}
