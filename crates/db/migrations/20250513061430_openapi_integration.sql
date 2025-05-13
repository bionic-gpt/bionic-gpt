-- migrate:up

ALTER TABLE integrations
    ADD COLUMN definition JSONB,
    ADD COLUMN api_key VARCHAR;

-- migrate:down

ALTER TABLE integrations
DROP COLUMN definition,
DROP COLUMN api_key;