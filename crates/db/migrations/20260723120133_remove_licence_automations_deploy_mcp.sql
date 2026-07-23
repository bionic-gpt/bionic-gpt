-- migrate:up

ALTER TABLE IF EXISTS llm.chats DROP COLUMN IF EXISTS automation_run_id;

DROP TABLE IF EXISTS automation.automation_cron_triggers;
DROP TABLE IF EXISTS automation.automation_webhook_triggers;
DROP TABLE IF EXISTS automation.automation_runs;
DROP SCHEMA IF EXISTS automation;

DELETE FROM assistants.prompts WHERE prompt_type = 'Automation';

ALTER TABLE assistants.prompts
    ALTER COLUMN prompt_type DROP DEFAULT;

CREATE TYPE prompt_type_without_automation AS ENUM ('Assistant', 'Model');

ALTER TABLE assistants.prompts
    ALTER COLUMN prompt_type TYPE prompt_type_without_automation
    USING prompt_type::text::prompt_type_without_automation;

DROP TYPE prompt_type;
ALTER TYPE prompt_type_without_automation RENAME TO prompt_type;

ALTER TABLE assistants.prompts
    ALTER COLUMN prompt_type SET DEFAULT 'Assistant';

DROP TYPE IF EXISTS automation_run_status;

UPDATE integrations.openapi_specs
SET
    title = 'Postgres MCP Server',
    description = replace(
        description,
        'Deploy' || ' MCP Postgres server',
        'Postgres MCP server'
    ),
    spec = replace(
        replace(
            replace(
                spec::text,
                'Deploy' || ' MCP Postgres Server',
                'Postgres MCP Server'
            ),
            'Deploy' || ' MCP Postgres server',
            'Postgres MCP server'
        ),
        'Deploy' || ' Postgres MCP server',
        'Postgres MCP server'
    )::jsonb
WHERE slug = 'postgres';

-- migrate:down

CREATE TYPE automation_run_status AS ENUM ('Pending', 'Running', 'Succeeded', 'Failed');

ALTER TABLE llm.chats
    ADD COLUMN IF NOT EXISTS automation_run_id INTEGER;

CREATE SCHEMA IF NOT EXISTS automation;

CREATE TABLE automation.automation_runs (
    id SERIAL PRIMARY KEY,
    prompt_id INTEGER REFERENCES assistants.prompts(id) ON DELETE CASCADE,
    conversation_id INTEGER,
    status automation_run_status NOT NULL DEFAULT 'Pending',
    error TEXT,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE TABLE automation.automation_cron_triggers (
    id SERIAL PRIMARY KEY,
    prompt_id INTEGER NOT NULL REFERENCES assistants.prompts(id) ON DELETE CASCADE,
    cron_expression TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE automation.automation_webhook_triggers (
    id SERIAL PRIMARY KEY,
    prompt_id INTEGER NOT NULL REFERENCES assistants.prompts(id) ON DELETE CASCADE,
    webhook_secret TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
