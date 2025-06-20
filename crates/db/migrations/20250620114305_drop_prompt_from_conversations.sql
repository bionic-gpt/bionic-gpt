-- migrate:up
ALTER TABLE conversations DROP COLUMN prompt_id;

-- migrate:down
ALTER TABLE conversations ADD COLUMN prompt_id VARCHAR;
