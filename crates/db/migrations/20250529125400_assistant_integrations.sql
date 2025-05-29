-- migrate:up

CREATE TABLE prompt_integration (
    prompt_id INT NOT NULL, 
    integration_id INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_prompt_integration_prompt FOREIGN KEY(prompt_id)
        REFERENCES prompts(id) ON DELETE CASCADE,

    CONSTRAINT FK_prompt_integration_integration FOREIGN KEY(integration_id)
        REFERENCES integrations(id) ON DELETE CASCADE,

    UNIQUE(prompt_id, integration_id)
);

-- Give access to the application user
GRANT SELECT, INSERT, UPDATE, DELETE ON prompt_integration TO bionic_application;

-- Give access to the readonly user
GRANT SELECT ON prompt_integration TO bionic_readonly;

-- migrate:down
DROP TABLE prompt_integration;