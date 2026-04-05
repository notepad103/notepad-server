-- 保证 notes.section_id 在 sections 中都有对应行，再添加外键
INSERT INTO sections (id, label, sort_order, created_at)
VALUES (
    'all',
    '全部',
    0,
    (EXTRACT(EPOCH FROM NOW()) * 1000)::BIGINT
)
ON CONFLICT (id) DO NOTHING;

INSERT INTO sections (id, label, sort_order, created_at)
SELECT DISTINCT n.section_id,
       n.section_id,
       0,
       (EXTRACT(EPOCH FROM NOW()) * 1000)::BIGINT
FROM notes n
WHERE NOT EXISTS (SELECT 1 FROM sections s WHERE s.id = n.section_id);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'fk_notes_section_id'
    ) THEN
        ALTER TABLE notes
        ADD CONSTRAINT fk_notes_section_id
        FOREIGN KEY (section_id) REFERENCES sections (id) ON DELETE RESTRICT;
    END IF;
END $$;
