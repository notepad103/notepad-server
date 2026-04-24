use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use bytes::Bytes;
use futures::StreamExt;
use rig::agent::MultiTurnStreamItem;
use rig::streaming::StreamedAssistantContent;

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::{
    CreateAgentRequest, CreateNoteRequest, FetchHtmlRequest, NoteResponse, NoteSummary,
    UpdateNoteRequest,
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
    let mut is_reasoning_delta = true;
    let byte_stream = stream.filter_map(move |item| {
        match item {
            Ok(StreamedAssistantContent::Text(text)) => {
                if is_reasoning_delta {
                    is_reasoning_delta = false;
                    let mut some_text = String::from("====text==== ");
                    some_text.push_str(&text.text);
                    print!("{}", some_text);
                    let _ = std::io::Write::flush(&mut std::io::stdout());
                    futures::future::ready(Some(Ok::<Bytes, String>(Bytes::from(some_text))))
                } else {
                    print!("{}", text.text);
                    let _ = std::io::Write::flush(&mut std::io::stdout());
                    futures::future::ready(Some(Ok::<Bytes, String>(Bytes::from(text.text))))
                }
            }
            Ok(StreamedAssistantContent::ReasoningDelta { reasoning, .. }) => {
                futures::future::ready(Some(Ok::<Bytes, String>(Bytes::from(reasoning))))
            }
            Ok(_) => futures::future::ready(None),
            Err(e) => futures::future::ready(Some(Err(e.to_string()))),
        }
    });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/plain; charset=utf-8")
        .body(Body::from_stream(byte_stream))
        .unwrap())
}

pub async fn create_agent(Json(req): Json<CreateAgentRequest>) -> Result<Response, AppError> {
    let stream = services::create_agent(&req.prompt).await?;
    let byte_stream = stream.filter_map(move |item| match item {
        Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(text))) => {
            futures::future::ready(Some(Ok::<Bytes, String>(Bytes::from(text.text))))
        }
        Ok(MultiTurnStreamItem::StreamAssistantItem(
            StreamedAssistantContent::ReasoningDelta { reasoning, .. },
        )) => {
            futures::future::ready(Some(Ok::<Bytes, String>(Bytes::from(reasoning))))
        }
        Ok(MultiTurnStreamItem::FinalResponse(res)) => {
            futures::future::ready(Some(Ok::<Bytes, String>(Bytes::from(res.response().to_string()))))
        }
        Ok(_) => futures::future::ready(None),
        Err(e) => futures::future::ready(Some(Err(e.to_string()))),
    });
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/plain; charset=utf-8")
        .body(Body::from_stream(byte_stream))
        .unwrap())
}
