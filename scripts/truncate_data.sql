-- 清空业务表数据，保留表结构与 _sqlx_migrations。
-- 依赖外键顺序由 CASCADE 处理；RESTART IDENTITY 重置 users.id 序列。

BEGIN;

TRUNCATE TABLE notes, sections, users RESTART IDENTITY CASCADE;

COMMIT;
