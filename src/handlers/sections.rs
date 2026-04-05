use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::{CreateSectionRequest, SectionResponse, UpdateSectionRequest};
use crate::services;
use crate::state::AppState;

pub async fn list_sections(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<SectionResponse>>), AppError> {
    let list = services::list_sections(&state.db, user_id).await?;
    Ok((StatusCode::OK, Json(list)))
}

pub async fn create_section(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(req): Json<CreateSectionRequest>,
) -> Result<(StatusCode, Json<SectionResponse>), AppError> {
    let section = services::create_section(&state.db, user_id, req).await?;
    Ok((StatusCode::CREATED, Json(section)))
}

pub async fn update_section(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateSectionRequest>,
) -> Result<(StatusCode, Json<SectionResponse>), AppError> {
    let section = services::update_section(&state.db, user_id, &id, req).await?;
    Ok((StatusCode::OK, Json(section)))
}

pub async fn delete_section(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    services::delete_section(&state.db, user_id, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}
