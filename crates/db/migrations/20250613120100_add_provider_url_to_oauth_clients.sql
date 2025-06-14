-- migrate:up
ALTER TABLE oauth_clients ADD COLUMN provider_url TEXT;

-- migrate:down
ALTER TABLE oauth_clients DROP COLUMN provider_url;
