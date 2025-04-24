-- migrate:up
ALTER TABLE chats RENAME COLUMN function_call TO tool_calls;
ALTER TABLE chats RENAME COLUMN function_call_results TO tool_call_results;

-- migrate:down
ALTER TABLE chats RENAME COLUMN tool_calls TO function_call;
ALTER TABLE chats RENAME COLUMN tool_call_results TO function_call_results;
