-- migrate:up
DO $$
BEGIN
    IF to_regclass('integrations.openapi_spec_selections') IS NOT NULL THEN
        DELETE FROM integrations.openapi_spec_selections
        WHERE category::TEXT = 'CodeSandbox';
    END IF;

    IF to_regclass('integrations.openapi_spec_api_keys') IS NOT NULL AND to_regclass('integrations.openapi_specs') IS NOT NULL THEN
        DELETE FROM integrations.openapi_spec_api_keys
        WHERE openapi_spec_id IN (
            SELECT id FROM integrations.openapi_specs WHERE category::TEXT = 'CodeSandbox'
        );
    END IF;

    IF to_regclass('integrations.openapi_specs') IS NOT NULL THEN
        DELETE FROM integrations.openapi_specs
        WHERE category::TEXT = 'CodeSandbox';
    END IF;
END $$;

ALTER TABLE IF EXISTS integrations.openapi_specs
ALTER COLUMN category DROP DEFAULT;

ALTER TABLE IF EXISTS integrations.openapi_spec_selections
ALTER COLUMN category TYPE TEXT
USING category::TEXT;

ALTER TABLE IF EXISTS integrations.openapi_specs
ALTER COLUMN category TYPE TEXT
USING category::TEXT;

DROP TYPE IF EXISTS openapi_spec_category;

CREATE TYPE openapi_spec_category AS ENUM (
    'WebSearch',
    'Application'
);

ALTER TABLE IF EXISTS integrations.openapi_spec_selections
ALTER COLUMN category TYPE openapi_spec_category
USING category::openapi_spec_category;

ALTER TABLE IF EXISTS integrations.openapi_specs
ALTER COLUMN category TYPE openapi_spec_category
USING category::openapi_spec_category;

ALTER TABLE IF EXISTS integrations.openapi_specs
ALTER COLUMN category SET DEFAULT 'Application';

-- migrate:down
ALTER TABLE IF EXISTS integrations.openapi_specs
ALTER COLUMN category DROP DEFAULT;

ALTER TABLE IF EXISTS integrations.openapi_spec_selections
ALTER COLUMN category TYPE TEXT
USING category::TEXT;

ALTER TABLE IF EXISTS integrations.openapi_specs
ALTER COLUMN category TYPE TEXT
USING category::TEXT;

DROP TYPE IF EXISTS openapi_spec_category;

CREATE TYPE openapi_spec_category AS ENUM (
    'WebSearch',
    'Application',
    'CodeSandbox'
);

ALTER TABLE IF EXISTS integrations.openapi_spec_selections
ALTER COLUMN category TYPE openapi_spec_category
USING category::openapi_spec_category;

ALTER TABLE IF EXISTS integrations.openapi_specs
ALTER COLUMN category TYPE openapi_spec_category
USING category::openapi_spec_category;

ALTER TABLE IF EXISTS integrations.openapi_specs
ALTER COLUMN category SET DEFAULT 'Application';
