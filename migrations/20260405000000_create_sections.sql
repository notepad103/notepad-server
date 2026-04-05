-- 笔记分区表（与 notes.section_id 对应）
CREATE TABLE IF NOT EXISTS sections (
    id         TEXT PRIMARY KEY,
    label      TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at BIGINT NOT NULL
);
