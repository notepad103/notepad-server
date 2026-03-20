-- users 新增 password 字段（迁移已执行过的情况下新增列）
ALTER TABLE users
ADD COLUMN IF NOT EXISTS password TEXT;

