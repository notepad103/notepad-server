use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub preview: String,
    pub body: String,
    pub section_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub title: String,
    pub preview: String,
    pub body: String,
    pub section_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NoteSummary {
    pub id: String,
    pub title: String,
    pub preview: String,
    pub section_id: String,
    pub updated_at: i64,
}

#[derive(Debug, Serialize)]
pub struct NoteResponse {
    pub id: String,
    pub user_id: i32,
    pub title: String,
    pub preview: String,
    pub body: String,
    pub section_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct FetchHtmlRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct FetchHtmlResponse {
    pub html: String,
}