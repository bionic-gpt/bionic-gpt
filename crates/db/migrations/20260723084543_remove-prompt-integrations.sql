-- migrate:up
DROP TABLE IF EXISTS integrations.prompt_integration;


-- migrate:down
CREATE TABLE IF NOT EXISTS integrations.prompt_integration (
    id SERIAL PRIMARY KEY,
    prompt_id INT NOT NULL,
    integration_id INT NOT NULL,
    api_connection_id INT,
    oauth2_connection_id INT,
    CONSTRAINT FK_prompt_integration_prompt
        FOREIGN KEY(prompt_id) REFERENCES assistants.prompts(id) ON DELETE CASCADE,
    CONSTRAINT FK_prompt_integration_integration
        FOREIGN KEY(integration_id) REFERENCES integrations.integrations(id) ON DELETE CASCADE,
    CONSTRAINT FK_prompt_integration_api_connection
        FOREIGN KEY(api_connection_id) REFERENCES integrations.api_key_connections(id) ON DELETE CASCADE,
    CONSTRAINT FK_prompt_integration_oauth2_connection
        FOREIGN KEY(oauth2_connection_id) REFERENCES integrations.oauth2_connections(id) ON DELETE CASCADE,
    CONSTRAINT CHK_prompt_integration_single_connection
        CHECK (
            (api_connection_id IS NULL AND oauth2_connection_id IS NULL)
            OR (api_connection_id IS NOT NULL AND oauth2_connection_id IS NULL)
            OR (api_connection_id IS NULL AND oauth2_connection_id IS NOT NULL)
        ),
    UNIQUE(prompt_id, integration_id)
);

GRANT SELECT, INSERT, UPDATE, DELETE ON integrations.prompt_integration TO application_user;
GRANT USAGE, SELECT ON integrations.prompt_integration_id_seq TO application_user;
GRANT SELECT ON integrations.prompt_integration TO application_readonly;
GRANT SELECT ON integrations.prompt_integration_id_seq TO application_readonly;
