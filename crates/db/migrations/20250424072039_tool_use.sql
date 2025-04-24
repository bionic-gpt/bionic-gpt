-- migrate:up
ALTER TABLE chats RENAME COLUMN function_call TO tool_calls;
ALTER TABLE chats RENAME COLUMN function_call_results TO tool_call_results;

ALTER TABLE integrations
    DROP COLUMN base_url,
    ADD COLUMN configuration JSONB;

CREATE TYPE integration_status AS ENUM ('Configured', 'AwaitingConfiguration');

ALTER TABLE integrations
    ADD COLUMN integration_status integration_status NOT NULL DEFAULT 'AwaitingConfiguration';

INSERT INTO integrations(
    name,
    integration_type,
    integration_status
) VALUES (
    'get_current_time_and_date',
    'BuiltIn',
    'Configured'
);

-- migrate:down
ALTER TABLE chats RENAME COLUMN tool_calls TO function_call;
ALTER TABLE chats RENAME COLUMN tool_call_results TO function_call_results;

DELETE FROM integrations;

ALTER TABLE integrations
    DROP COLUMN configuration,
    ADD COLUMN base_url VARCHAR;

ALTER TABLE integrations
    DROP COLUMN integration_status;

DROP TYPE integration_status;  
