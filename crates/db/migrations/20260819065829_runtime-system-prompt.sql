-- migrate:up

CREATE TABLE ops.runtime_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

GRANT SELECT, INSERT, UPDATE, DELETE ON ops.runtime_settings TO application_user;
GRANT SELECT ON ops.runtime_settings TO application_readonly;

SELECT updated_at('ops.runtime_settings');

INSERT INTO ops.runtime_settings (key, value)
VALUES (
    'default_system_prompt',
    'Use tools when the user asks for current, live, external, account-specific, or connected-system information.
For prices, news, market data, web pages, search results, files, integrations, connected data, inboxes, email, calendar, CRM, tickets, or account data, do not answer from memory.
When a request might be answered by a connected system, call search_tool_functions before saying you do not have access.
Use search_tool_functions to find available callable functions, inspect or describe the function if needed, then use the appropriate runtime tool to execute it.'
);

-- migrate:down

DROP TABLE ops.runtime_settings;
