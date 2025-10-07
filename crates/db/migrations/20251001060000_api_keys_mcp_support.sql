-- migrate:up

ALTER TABLE api_keys ALTER COLUMN prompt_id DROP NOT NULL;

UPDATE api_keys
SET api_key = encode(digest(api_key, 'sha256'), 'hex');

CREATE UNIQUE INDEX IF NOT EXISTS unique_api_key_per_team
ON api_keys (team_id, api_key);

-- migrate:down

DROP INDEX IF EXISTS unique_api_key_per_team;

ALTER TABLE api_keys ALTER COLUMN prompt_id SET NOT NULL;
