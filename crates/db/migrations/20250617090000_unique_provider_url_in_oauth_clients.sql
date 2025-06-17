-- migrate:up
-- Remove duplicate provider_url rows keeping the one with the smallest id
DELETE FROM oauth_clients a
USING oauth_clients b
WHERE a.provider_url = b.provider_url
  AND a.id > b.id;

ALTER TABLE oauth_clients
    ADD CONSTRAINT unique_provider_url UNIQUE(provider_url);

-- migrate:down
ALTER TABLE oauth_clients
    DROP CONSTRAINT IF EXISTS unique_provider_url;
