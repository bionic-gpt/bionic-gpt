-- migrate:up
ALTER TABLE datasets ADD COLUMN created_by INT NOT NULL DEFAULT 0;
ALTER TABLE prompts ADD COLUMN created_by INT NOT NULL DEFAULT 0;
ALTER TABLE datasets ALTER COLUMN created_by DROP DEFAULT;
ALTER TABLE prompts ALTER COLUMN created_by DROP DEFAULT;
-- migrate:down
ALTER TABLE datasets DROP COLUMN created_by;
ALTER TABLE prompts DROP COLUMN created_by;