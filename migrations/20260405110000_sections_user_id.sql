-- sections 按用户隔离：复合主键 (user_id, id)，notes 以 (user_id, section_id) 引用
ALTER TABLE notes
DROP CONSTRAINT IF EXISTS fk_notes_section_id;

DELETE FROM sections;

ALTER TABLE sections
DROP CONSTRAINT sections_pkey;

ALTER TABLE sections
ADD COLUMN user_id INTEGER REFERENCES users (id) ON DELETE CASCADE;

INSERT INTO sections (user_id, id, label, sort_order, created_at)
SELECT DISTINCT n.user_id,
       n.section_id,
       n.section_id,
       0,
       (EXTRACT(EPOCH FROM NOW()) * 1000)::BIGINT
FROM notes n;

INSERT INTO sections (user_id, id, label, sort_order, created_at)
SELECT u.id,
       'all',
       '全部',
       0,
       (EXTRACT(EPOCH FROM NOW()) * 1000)::BIGINT
FROM users u
WHERE NOT EXISTS (
    SELECT 1
    FROM sections s
    WHERE s.user_id = u.id
      AND s.id = 'all'
);

ALTER TABLE sections
ALTER COLUMN user_id SET NOT NULL;

ALTER TABLE sections
ADD CONSTRAINT sections_pkey PRIMARY KEY (user_id, id);

CREATE INDEX IF NOT EXISTS idx_sections_user_id ON sections (user_id);

ALTER TABLE notes
ADD CONSTRAINT fk_notes_user_section
FOREIGN KEY (user_id, section_id) REFERENCES sections (user_id, id) ON DELETE RESTRICT;
