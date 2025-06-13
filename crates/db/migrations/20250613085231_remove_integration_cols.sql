-- migrate:up
ALTER TABLE integrations DROP COLUMN integration_status;
ALTER TABLE integrations DROP COLUMN configuration;
DROP TYPE integration_status;

-- migrate:down
CREATE TYPE integration_status AS ENUM ('Configured', 'AwaitingConfiguration');
ALTER TABLE integrations ADD COLUMN integration_status integration_status NOT NULL DEFAULT 'AwaitingConfiguration';
ALTER TABLE integrations ADD COLUMN configuration JSONB;
