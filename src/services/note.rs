use chrono::Utc;
use sqlx::{PgPool, Row};
use tracing::info;

use crate::error::AppError;
use crate::models::{CreateNoteRequest, NoteResponse, NoteSummary, UpdateNoteRequest};

fn gen_note_id(user_id: i32) -> String {
    let ts = Utc::now().timestamp_millis();
    let suffix: u32 = rand::random::<u32>() % 10_000;
    format!("n-{user_id}-{ts}-{suffix:04}")
}

pub async fn create_note(
    pool: &PgPool,
    user_id: i32,
    req: CreateNoteRequest,
) -> Result<NoteResponse, AppError> {
    let now = Utc::now().timestamp_millis();
    let id = gen_note_id(user_id);
    let section_id = req.section_id.unwrap_or_else(|| "all".to_string());

    let row = sqlx::query(
        "INSERT INTO notes (id, user_id, title, preview, body, section_id, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING id, user_id, title, preview, body, section_id, created_at, updated_at",
    )
    .bind(&id)
    .bind(user_id)
    .bind(&req.title)
    .bind(&req.preview)
    .bind(&req.body)
    .bind(&section_id)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    let note = NoteResponse {
        id: row.get(0),
        user_id: row.get(1),
        title: row.get(2),
        preview: row.get(3),
        body: row.get(4),
        section_id: row.get(5),
        created_at: row.get(6),
        updated_at: row.get(7),
    };
    info!(note_id = %note.id, user_id = note.user_id, "note created");
    Ok(note)
}

pub async fn list_notes(pool: &PgPool, user_id: i32) -> Result<Vec<NoteSummary>, AppError> {
    let rows = sqlx::query(
        "SELECT id, title, preview, section_id, updated_at
         FROM notes
         WHERE user_id = $1
         ORDER BY updated_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| NoteSummary {
            id: row.get(0),
            title: row.get(1),
            preview: row.get(2),
            section_id: row.get(3),
            updated_at: row.get(4),
        })
        .collect())
}

pub async fn get_note(pool: &PgPool, user_id: i32, note_id: &str) -> Result<NoteResponse, AppError> {
    let row = sqlx::query(
        "SELECT id, user_id, title, preview, body, section_id, created_at, updated_at
         FROM notes
         WHERE id = $1 AND user_id = $2",
    )
    .bind(note_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::BadRequest("笔记不存在".into()));
    };

    Ok(NoteResponse {
        id: row.get(0),
        user_id: row.get(1),
        title: row.get(2),
        preview: row.get(3),
        body: row.get(4),
        section_id: row.get(5),
        created_at: row.get(6),
        updated_at: row.get(7),
    })
}

pub async fn update_note(
    pool: &PgPool,
    user_id: i32,
    note_id: &str,
    req: UpdateNoteRequest,
) -> Result<NoteResponse, AppError> {
    let section_id = req.section_id.unwrap_or_else(|| "all".to_string());
    let now = Utc::now().timestamp_millis();

    let row = sqlx::query(
        "UPDATE notes
         SET title = $1, preview = $2, body = $3, section_id = $4, updated_at = $5
         WHERE id = $6 AND user_id = $7
         RETURNING id, user_id, title, preview, body, section_id, created_at, updated_at",
    )
    .bind(&req.title)
    .bind(&req.preview)
    .bind(&req.body)
    .bind(&section_id)
    .bind(now)
    .bind(note_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::BadRequest("笔记不存在".into()));
    };

    Ok(NoteResponse {
        id: row.get(0),
        user_id: row.get(1),
        title: row.get(2),
        preview: row.get(3),
        body: row.get(4),
        section_id: row.get(5),
        created_at: row.get(6),
        updated_at: row.get(7),
    })
}

pub async fn delete_note(pool: &PgPool, user_id: i32, note_id: &str) -> Result<(), AppError> {
    let r = sqlx::query("DELETE FROM notes WHERE id = $1 AND user_id = $2")
        .bind(note_id)
        .bind(user_id)
        .execute(pool)
        .await?;

    if r.rows_affected() == 0 {
        return Err(AppError::BadRequest("笔记不存在".into()));
    }
    info!(note_id = %note_id, user_id = user_id, "note deleted");
    Ok(())
}
