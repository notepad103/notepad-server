use axum::{
    Json,
    extract::{Path, State},
};

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::{CreateNoteRequest, NoteResponse, NoteSummary, UpdateNoteRequest};
use crate::response::ApiResponse;
use crate::services;
use crate::state::AppState;

pub async fn create_note(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(req): Json<CreateNoteRequest>,
) -> Result<ApiResponse<NoteResponse>, AppError> {
    let note = services::create_note(&state.db, user_id, req).await?;
    Ok(ApiResponse(note))
}

pub async fn list_notes(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
) -> Result<ApiResponse<Vec<NoteSummary>>, AppError> {
    let notes = services::list_notes(&state.db, user_id).await?;
    Ok(ApiResponse(notes))
}

pub async fn get_note(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<ApiResponse<NoteResponse>, AppError> {
    let note = services::get_note(&state.db, user_id, &id).await?;
    Ok(ApiResponse(note))
}

pub async fn update_note(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateNoteRequest>,
) -> Result<ApiResponse<NoteResponse>, AppError> {
    let note = services::update_note(&state.db, user_id, &id, req).await?;
    Ok(ApiResponse(note))
}

pub async fn delete_note(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<ApiResponse<()>, AppError> {
    services::delete_note(&state.db, user_id, &id).await?;
    Ok(ApiResponse(()))
}
