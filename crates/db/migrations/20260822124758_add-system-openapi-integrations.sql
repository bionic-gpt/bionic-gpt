-- migrate:up

ALTER TABLE integrations.openapi_specs
ADD COLUMN is_system BOOLEAN NOT NULL DEFAULT FALSE;

UPDATE integrations.openapi_specs
SET is_system = TRUE
WHERE slug = 'document-conversion-api';

-- migrate:down

ALTER TABLE integrations.openapi_specs
DROP COLUMN is_system;
