use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateSectionRequest {
    pub label: String,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSectionRequest {
    pub label: String,
    pub sort_order: i32,
}

#[derive(Debug, Serialize)]
pub struct SectionResponse {
    pub id: String,
    pub label: String,
    pub sort_order: i32,
    pub created_at: i64,
}
