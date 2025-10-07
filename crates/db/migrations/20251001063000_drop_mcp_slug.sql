-- migrate:up

ALTER TABLE api_keys DROP CONSTRAINT IF EXISTS api_keys_prompt_or_slug;
ALTER TABLE api_keys DROP COLUMN IF EXISTS mcp_slug;

-- migrate:down

ALTER TABLE api_keys ADD COLUMN mcp_slug TEXT;
ALTER TABLE api_keys
    ADD CONSTRAINT api_keys_prompt_or_slug
    CHECK ((prompt_id IS NOT NULL) <> (mcp_slug IS NOT NULL));
