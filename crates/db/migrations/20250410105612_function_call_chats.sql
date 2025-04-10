-- migrate:up
ALTER TABLE chats
    ADD COLUMN function_call VARCHAR,
    ADD COLUMN function_call_results VARCHAR;

-- migrate:down
ALTER TABLE chats
    DROP COLUMN function_call,
    DROP COLUMN function_call_results;


