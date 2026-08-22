-- migrate:up

ALTER TABLE integrations.openapi_specs
ADD COLUMN is_system BOOLEAN NOT NULL DEFAULT FALSE;

UPDATE integrations.openapi_specs
SET is_system = TRUE
WHERE slug = 'xberg-doc-engine';

-- migrate:down

ALTER TABLE integrations.openapi_specs
DROP COLUMN is_system;
