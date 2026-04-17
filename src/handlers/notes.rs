use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use bytes::Bytes;
use futures::StreamExt;
use rig_core::streaming::StreamedAssistantContent;

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::{
    CreateNoteRequest, FetchHtmlRequest, NoteResponse, NoteSummary, UpdateNoteRequest,
};
use crate::services;
use crate::state::AppState;

pub async fn create_note(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Json(req): Json<CreateNoteRequest>,
) -> Result<(StatusCode, Json<NoteResponse>), AppError> {
    let note = services::create_note(&state.db, user_id, req).await?;
    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn list_notes(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<NoteSummary>>), AppError> {
    let notes = services::list_notes(&state.db, user_id).await?;
    Ok((StatusCode::OK, Json(notes)))
}

pub async fn get_note(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<NoteResponse>), AppError> {
    let note = services::get_note(&state.db, user_id, &id).await?;
    Ok((StatusCode::OK, Json(note)))
}

pub async fn update_note(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateNoteRequest>,
) -> Result<(StatusCode, Json<NoteResponse>), AppError> {
    let note = services::update_note(&state.db, user_id, &id, req).await?;
    Ok((StatusCode::OK, Json(note)))
}

pub async fn delete_note(
    AuthUser(user_id): AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    services::delete_note(&state.db, user_id, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn fetch_html(Json(req): Json<FetchHtmlRequest>) -> Result<Response, AppError> {
    let stream = services::fetch_html(&req.url).await?;
    let byte_stream = stream.filter_map(|item| async move {
        match item {
            Ok(StreamedAssistantContent::Text(text)) => {
                print!("{}", text.text);
                let _ = std::io::Write::flush(&mut std::io::stdout());
                Some(Ok::<Bytes, String>(Bytes::from(text.text)))
            }
            Ok(StreamedAssistantContent::ReasoningDelta { reasoning, .. }) => {
                Some(Ok::<Bytes, String>(Bytes::from(reasoning)))
            }
            Ok(_) => None,
            Err(e) => Some(Err(e.to_string())),
        }
    });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/plain; charset=utf-8")
        .body(Body::from_stream(byte_stream))
        .unwrap())
}
