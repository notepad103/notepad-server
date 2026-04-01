-- notes 新增 user_id 字段，并关联 users(id)
ALTER TABLE notes
ADD COLUMN IF NOT EXISTS user_id INTEGER;

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM notes WHERE user_id IS NULL) THEN
        RAISE EXCEPTION 'notes.user_id contains NULL values, please backfill before setting NOT NULL';
    END IF;
END $$;

ALTER TABLE notes
ALTER COLUMN user_id SET NOT NULL;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'fk_notes_user_id'
    ) THEN
        ALTER TABLE notes
        ADD CONSTRAINT fk_notes_user_id
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_notes_user_id ON notes (user_id);
