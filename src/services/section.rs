use chrono::Utc;
use sqlx::{PgPool, Row};

use crate::error::AppError;
use crate::models::{CreateSectionRequest, SectionResponse, UpdateSectionRequest};

fn gen_section_id(user_id: i32) -> String {
    let ts = Utc::now().timestamp_millis();
    let suffix: u32 = rand::random::<u32>() % 10_000;
    format!("s-{user_id}-{ts}-{suffix:04}")
}

pub async fn list_sections(pool: &PgPool, user_id: i32) -> Result<Vec<SectionResponse>, AppError> {
    let rows = sqlx::query(
        "SELECT id, label, sort_order, created_at
         FROM sections
         WHERE user_id = $1
         ORDER BY sort_order ASC, id ASC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| SectionResponse {
            id: row.get(0),
            label: row.get(1),
            sort_order: row.get(2),
            created_at: row.get(3),
        })
        .collect())
}

pub async fn create_section(
    pool: &PgPool,
    user_id: i32,
    req: CreateSectionRequest,
) -> Result<SectionResponse, AppError> {
    let now = Utc::now().timestamp_millis();
    let id = gen_section_id(user_id);
    let sort_order = req.sort_order.unwrap_or(0);

    let row = sqlx::query(
        "INSERT INTO sections (user_id, id, label, sort_order, created_at)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, label, sort_order, created_at",
    )
    .bind(user_id)
    .bind(&id)
    .bind(&req.label)
    .bind(sort_order)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(SectionResponse {
        id: row.get(0),
        label: row.get(1),
        sort_order: row.get(2),
        created_at: row.get(3),
    })
}

pub async fn update_section(
    pool: &PgPool,
    user_id: i32,
    section_id: &str,
    req: UpdateSectionRequest,
) -> Result<SectionResponse, AppError> {
    let row = sqlx::query(
        "UPDATE sections
         SET label = $1, sort_order = $2
         WHERE user_id = $3 AND id = $4
         RETURNING id, label, sort_order, created_at",
    )
    .bind(&req.label)
    .bind(req.sort_order)
    .bind(user_id)
    .bind(section_id)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Err(AppError::BadRequest("分区不存在".into()));
    };

    Ok(SectionResponse {
        id: row.get(0),
        label: row.get(1),
        sort_order: row.get(2),
        created_at: row.get(3),
    })
}

pub async fn delete_section(pool: &PgPool, user_id: i32, section_id: &str) -> Result<(), AppError> {
    if section_id == "all" {
        return Err(AppError::BadRequest("不能删除默认分区".into()));
    }

    let r = sqlx::query("DELETE FROM sections WHERE user_id = $1 AND id = $2")
        .bind(user_id)
        .bind(section_id)
        .execute(pool)
        .await?;

    if r.rows_affected() == 0 {
        return Err(AppError::BadRequest("分区不存在".into()));
    }

    Ok(())
}
