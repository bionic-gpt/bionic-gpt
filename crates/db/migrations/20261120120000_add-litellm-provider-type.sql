-- migrate:up transaction:false
-- LiteLLM is an OpenAI-compatible gateway in front of many model providers.
-- A new enum value cannot be used until it is committed, so the provider
-- seed lives in the next migration.
ALTER TYPE model_provider ADD VALUE IF NOT EXISTS 'LiteLLM';

-- migrate:down
-- Postgres cannot drop a single enum value; the unused label stays.
SELECT 1;
