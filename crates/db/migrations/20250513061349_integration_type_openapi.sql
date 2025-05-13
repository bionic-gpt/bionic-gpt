-- migrate:up
ALTER TYPE integration_type ADD VALUE IF NOT EXISTS 'OpenAPI';

-- migrate:down
