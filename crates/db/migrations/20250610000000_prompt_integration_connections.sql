-- migrate:up

-- Add nullable connection reference columns
ALTER TABLE prompt_integration 
ADD COLUMN api_connection_id INT,
ADD COLUMN oauth2_connection_id INT;

-- Add foreign key constraints
ALTER TABLE prompt_integration
ADD CONSTRAINT FK_prompt_integration_api_connection 
    FOREIGN KEY(api_connection_id) 
    REFERENCES api_key_connections(id) ON DELETE SET NULL,

ADD CONSTRAINT FK_prompt_integration_oauth2_connection 
    FOREIGN KEY(oauth2_connection_id) 
    REFERENCES oauth2_connections(id) ON DELETE SET NULL;

-- Prevent both connection types from being set simultaneously
ALTER TABLE prompt_integration
ADD CONSTRAINT CHK_prompt_integration_single_connection
    CHECK (NOT (api_connection_id IS NOT NULL AND oauth2_connection_id IS NOT NULL));

-- Note: Additional validation that connections belong to the same integration
-- will be handled at the application level, as PostgreSQL CHECK constraints
-- cannot contain subqueries.

-- migrate:down
ALTER TABLE prompt_integration
DROP CONSTRAINT IF EXISTS CHK_prompt_integration_single_connection,
DROP CONSTRAINT IF EXISTS FK_prompt_integration_oauth2_connection,
DROP CONSTRAINT IF EXISTS FK_prompt_integration_api_connection,
DROP COLUMN IF EXISTS oauth2_connection_id,
DROP COLUMN IF EXISTS api_connection_id;