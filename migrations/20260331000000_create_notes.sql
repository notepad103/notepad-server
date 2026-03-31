-- 笔记表（PostgreSQL）
CREATE TABLE IF NOT EXISTS notes (
    id         TEXT PRIMARY KEY,
    title      TEXT NOT NULL,
    preview    TEXT NOT NULL,
    body       TEXT NOT NULL,
    section_id TEXT NOT NULL DEFAULT 'all',
    created_at BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_notes_section_id ON notes (section_id);
CREATE INDEX IF NOT EXISTS idx_notes_updated_at ON notes (updated_at DESC);
