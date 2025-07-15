-- migrate:up
-- Remove rows where neither connection is set
DELETE FROM prompt_integration
WHERE api_connection_id IS NULL AND oauth2_connection_id IS NULL;

-- Update foreign key constraints to cascade deletes
ALTER TABLE prompt_integration
    DROP CONSTRAINT IF EXISTS FK_prompt_integration_api_connection,
    DROP CONSTRAINT IF EXISTS FK_prompt_integration_oauth2_connection;

ALTER TABLE prompt_integration
    ADD CONSTRAINT FK_prompt_integration_api_connection
        FOREIGN KEY(api_connection_id)
        REFERENCES api_key_connections(id) ON DELETE CASCADE,
    ADD CONSTRAINT FK_prompt_integration_oauth2_connection
        FOREIGN KEY(oauth2_connection_id)
        REFERENCES oauth2_connections(id) ON DELETE CASCADE;

-- migrate:down
ALTER TABLE prompt_integration
    DROP CONSTRAINT IF EXISTS FK_prompt_integration_api_connection,
    DROP CONSTRAINT IF EXISTS FK_prompt_integration_oauth2_connection;

ALTER TABLE prompt_integration
    ADD CONSTRAINT FK_prompt_integration_api_connection
        FOREIGN KEY(api_connection_id)
        REFERENCES api_key_connections(id) ON DELETE SET NULL,
    ADD CONSTRAINT FK_prompt_integration_oauth2_connection
        FOREIGN KEY(oauth2_connection_id)
        REFERENCES oauth2_connections(id) ON DELETE SET NULL;
