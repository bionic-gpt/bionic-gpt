-- migrate:up
ALTER TABLE prompts ADD COLUMN image_icon BYTEA;

-- migrate:down
ALTER TABLE prompts DROP COLUMN image_icon;
