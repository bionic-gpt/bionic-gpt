-- migrate:up
DELETE FROM datasets;
ALTER TABLE datasets ADD COLUMN created_by INT NOT NULL;
-- migrate:down
ALTER TABLE datasets DROP COLUMN created_by;