-- migrate:up
ALTER TABLE providers
    ADD COLUMN api_key_optional BOOLEAN NOT NULL DEFAULT false;

UPDATE providers
SET api_key_optional = true
WHERE name = 'Ollama';

-- migrate:down
ALTER TABLE providers
    DROP COLUMN api_key_optional;
