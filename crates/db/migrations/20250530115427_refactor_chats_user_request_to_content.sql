-- migrate:up

DROP TRIGGER IF EXISTS update_chats ON chats RESTRICT;

-- Create the chat_role enum type
CREATE TYPE chat_role AS ENUM (
    'User', 
    'Assistant', 
    'Tool', 
    'System', 
    'Developer'
);
COMMENT ON TYPE chat_role IS 'The role of the message sender in a chat conversation';

UPDATE prompts SET max_history_items = 99 WHERE max_history_items = 3;

-- Add the new columns to the chats table
ALTER TABLE chats 
    ADD COLUMN content VARCHAR,
    ADD COLUMN tool_call_id VARCHAR,
    ADD COLUMN role chat_role NOT NULL DEFAULT 'User';

-- Copy existing user_request data to the new content column and set role to 'User'
-- This preserves the encrypted data as-is
UPDATE chats SET
    content = user_request,
    role = 'User';

-- Insert assistant response entries for chats that have response data
-- This creates separate entries for assistant responses
INSERT INTO chats (
    conversation_id,
    status,
    content,
    user_request,
    prompt,
    prompt_id,
    role,
    created_at,
    updated_at,
    tokens_sent,
    tokens_received,
    time_taken_ms,
    tool_calls,
    tool_call_results
)
SELECT
    conversation_id,
    'Success'::chat_status,  -- Assistant responses indicate successful completion
    response,                -- Copy response to content
    '',
    prompt,
    prompt_id,
    'Assistant'::chat_role,  -- Set role to Assistant
    created_at,             -- Preserve original timestamps
    updated_at,
    tokens_sent,
    tokens_received,
    time_taken_ms,
    tool_calls,
    tool_call_results
FROM chats
WHERE response IS NOT NULL AND response != '';

-- Drop the old columns
ALTER TABLE chats
    DROP COLUMN request_embeddings,
    DROP COLUMN response,
    DROP COLUMN prompt,
    DROP COLUMN response_embeddings,
    DROP COLUMN user_request;

-- migrate:down

-- Recreate the old columns
ALTER TABLE chats ADD COLUMN user_request VARCHAR NOT NULL DEFAULT '';
ALTER TABLE chats ADD COLUMN request_embeddings VARCHAR;
ALTER TABLE chats ADD COLUMN response VARCHAR;
ALTER TABLE chats ADD COLUMN prompt VARCHAR NOT NULL DEFAULT '';
ALTER TABLE chats ADD COLUMN response_embeddings VARCHAR;

-- Copy user messages back to user_request
UPDATE chats SET user_request = COALESCE(content, '')
WHERE role = 'User';

-- Copy assistant responses back to response field
-- We need to match assistant responses with their corresponding user messages
-- by conversation_id, prompt_id, and timestamps
UPDATE chats SET response = (
    SELECT assistant_chats.content
    FROM chats assistant_chats
    WHERE assistant_chats.role = 'Assistant'
    AND assistant_chats.conversation_id = chats.conversation_id
    AND assistant_chats.prompt_id = chats.prompt_id
    AND assistant_chats.created_at = chats.created_at
    LIMIT 1
)
WHERE role = 'User';

-- Remove assistant entries (they're now consolidated back into user entries)
DELETE FROM chats WHERE role = 'Assistant';

-- Drop the new columns and enum type
ALTER TABLE chats
    DROP COLUMN content,
    DROP COLUMN tool_call_id,
    DROP COLUMN role;

DROP TYPE chat_role;