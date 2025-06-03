-- migrate:up
ALTER TABLE chats DROP COLUMN tool_call_results;

-- migrate:down
ALTER TABLE chats ADD COLUMN tool_call_results JSONB;
