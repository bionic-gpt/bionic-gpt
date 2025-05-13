-- migrate:up

ALTER TABLE integrations
    ADD COLUMN definition JSONB;

-- migrate:down

ALTER TABLE integrations
DROP COLUMN definition;