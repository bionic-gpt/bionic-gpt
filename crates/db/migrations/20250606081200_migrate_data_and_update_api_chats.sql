-- migrate:up

-- First, migrate existing data from chats table to token_usage_metrics
INSERT INTO token_usage_metrics (chat_id, type, tokens, duration_ms, created_at)
SELECT 
    id,
    'Prompt'::token_usage_type,
    tokens_sent,
    NULL, -- Duration only tracked for completions
    created_at
FROM chats 
WHERE tokens_sent > 0;

INSERT INTO token_usage_metrics (chat_id, type, tokens, duration_ms, created_at)
SELECT 
    id,
    'Completion'::token_usage_type,
    tokens_received,
    time_taken_ms,
    updated_at
FROM chats 
WHERE tokens_received > 0;

-- Migrate existing data from api_chats table to token_usage_metrics
INSERT INTO token_usage_metrics (api_key_id, type, tokens, duration_ms, created_at)
SELECT 
    api_key_id,
    'Prompt'::token_usage_type,
    tokens_sent,
    NULL,
    created_at
FROM api_chats 
WHERE tokens_sent > 0;

INSERT INTO token_usage_metrics (api_key_id, type, tokens, duration_ms, created_at)
SELECT 
    api_key_id,
    'Completion'::token_usage_type,
    tokens_received,
    time_taken_ms,
    updated_at
FROM api_chats 
WHERE tokens_received > 0;

-- Update api_chats table structure to match chats
ALTER TABLE api_chats 
    ADD COLUMN role chat_role NOT NULL DEFAULT 'User',
    ADD COLUMN tool_calls VARCHAR,
    ADD COLUMN content VARCHAR,
    ADD COLUMN tool_call_id VARCHAR;

-- Migrate existing api_chats data to new structure
UPDATE api_chats SET 
    content = prompt,
    role = 'User'::chat_role;

-- Insert assistant responses for api_chats that have responses
INSERT INTO api_chats (
    api_key_id,
    prompt,
    content,
    role,
    status,
    created_at,
    updated_at
)
SELECT 
    api_key_id,
    '' AS prompt,
    response,
    'Assistant'::chat_role,
    status,
    created_at,
    updated_at
FROM api_chats 
WHERE response IS NOT NULL AND response != '';

-- Drop the inference_metrics view first since it depends on token columns
DROP VIEW inference_metrics;

-- Remove old columns from both tables
ALTER TABLE chats
    DROP COLUMN tokens_sent,
    DROP COLUMN tokens_received,
    DROP COLUMN time_taken_ms;

ALTER TABLE api_chats
    DROP COLUMN prompt,
    DROP COLUMN response,
    DROP COLUMN tokens_sent,
    DROP COLUMN tokens_received,
    DROP COLUMN time_taken_ms;

-- Recreate the inference_metrics view using the new token_usage_metrics table
CREATE OR REPLACE VIEW inference_metrics AS
WITH combined_data AS (
    -- Get data from token_usage_metrics for UI chats
    SELECT
        tum.id,
        'Console'::inference_type AS inference_type,
        (SELECT model_id FROM prompts p WHERE p.id = c.prompt_id) AS model_id,
        (SELECT user_id FROM conversations conv WHERE conv.id = c.conversation_id) AS user_id,
        CASE WHEN tum.type = 'Prompt' THEN tum.tokens ELSE 0 END AS tokens_sent,
        CASE WHEN tum.type = 'Completion' THEN tum.tokens ELSE 0 END AS tokens_received,
        COALESCE(tum.duration_ms, 0) AS time_taken_ms,
        tum.created_at,
        tum.created_at AS updated_at
    FROM token_usage_metrics tum
    JOIN chats c ON tum.chat_id = c.id
    WHERE tum.chat_id IS NOT NULL
    
    UNION ALL
    
    -- Get data from token_usage_metrics for API calls
    SELECT
        tum.id,
        'API'::inference_type AS inference_type,
        (SELECT model_id FROM prompts p WHERE p.id IN (SELECT prompt_id FROM api_keys a WHERE a.id = tum.api_key_id)) AS model_id,
        (SELECT user_id FROM api_keys k WHERE k.id = tum.api_key_id) AS user_id,
        CASE WHEN tum.type = 'Prompt' THEN tum.tokens ELSE 0 END AS tokens_sent,
        CASE WHEN tum.type = 'Completion' THEN tum.tokens ELSE 0 END AS tokens_received,
        COALESCE(tum.duration_ms, 0) AS time_taken_ms,
        tum.created_at,
        tum.created_at AS updated_at
    FROM token_usage_metrics tum
    WHERE tum.api_key_id IS NOT NULL
),
recent_data AS (
    SELECT
        model_id,
        user_id,
        SUM(tokens_sent) AS tpm_sent,
        SUM(tokens_received) AS tpm_recv
    FROM combined_data
    WHERE created_at >= NOW() - INTERVAL '1 minute'
    GROUP BY model_id, user_id
)
SELECT
    model_id,
    user_id,
    tpm_sent,
    tpm_recv
FROM recent_data;

-- Grant permissions
GRANT SELECT ON inference_metrics TO bionic_application;
GRANT SELECT ON inference_metrics TO bionic_readonly;

-- migrate:down

-- Drop the updated inference_metrics view
DROP VIEW inference_metrics;

-- Restore old columns
ALTER TABLE chats
    ADD COLUMN tokens_sent INT NOT NULL DEFAULT 0,
    ADD COLUMN tokens_received INT NOT NULL DEFAULT 0,
    ADD COLUMN time_taken_ms INT NOT NULL DEFAULT 0;

ALTER TABLE api_chats
    ADD COLUMN prompt VARCHAR NOT NULL DEFAULT '',
    ADD COLUMN response VARCHAR,
    ADD COLUMN tokens_sent INT NOT NULL DEFAULT 0,
    ADD COLUMN tokens_received INT NOT NULL DEFAULT 0,
    ADD COLUMN time_taken_ms INT NOT NULL DEFAULT 0;

-- Restore data from token_usage_metrics
UPDATE chats SET 
    tokens_sent = (
        SELECT COALESCE(SUM(tokens), 0) 
        FROM token_usage_metrics 
        WHERE chat_id = chats.id AND type = 'Prompt'
    ),
    tokens_received = (
        SELECT COALESCE(SUM(tokens), 0) 
        FROM token_usage_metrics 
        WHERE chat_id = chats.id AND type = 'Completion'
    ),
    time_taken_ms = (
        SELECT COALESCE(MAX(duration_ms), 0) 
        FROM token_usage_metrics 
        WHERE chat_id = chats.id AND type = 'Completion'
    );

UPDATE api_chats SET 
    tokens_sent = (
        SELECT COALESCE(SUM(tokens), 0) 
        FROM token_usage_metrics 
        WHERE api_key_id = api_chats.api_key_id AND type = 'Prompt'
    ),
    tokens_received = (
        SELECT COALESCE(SUM(tokens), 0) 
        FROM token_usage_metrics 
        WHERE api_key_id = api_chats.api_key_id AND type = 'Completion'
    ),
    time_taken_ms = (
        SELECT COALESCE(MAX(duration_ms), 0) 
        FROM token_usage_metrics 
        WHERE api_key_id = api_chats.api_key_id AND type = 'Completion'
    );

-- Restore api_chats structure by copying assistant responses back to response field
-- We need to match assistant responses with their corresponding user messages
UPDATE api_chats SET response = (
    SELECT assistant_chats.content
    FROM api_chats assistant_chats
    WHERE assistant_chats.role = 'Assistant'
    AND assistant_chats.api_key_id = api_chats.api_key_id
    AND assistant_chats.created_at >= api_chats.created_at
    ORDER BY assistant_chats.created_at ASC
    LIMIT 1
)
WHERE role = 'User';

-- Copy user content back to prompt field
UPDATE api_chats SET prompt = content WHERE role = 'User';

-- Remove assistant entries (they're now consolidated back into user entries)
DELETE FROM api_chats WHERE role = 'Assistant';

-- Remove new columns from api_chats
ALTER TABLE api_chats
    DROP COLUMN role,
    DROP COLUMN tool_calls,
    DROP COLUMN content,
    DROP COLUMN tool_call_id;

-- Restore the original inference_metrics view
CREATE OR REPLACE VIEW inference_metrics AS
WITH combined_data AS (
    SELECT
        id,
        'API'::inference_type AS inference_type,
        (SELECT model_id FROM prompts p WHERE p.id IN (SELECT prompt_id FROM api_keys a WHERE a.id = api_key_id)) AS model_id,
        (SELECT user_id FROM api_keys k WHERE k.id = api_key_id) AS user_id,
        tokens_sent,
        tokens_received,
        time_taken_ms,
        created_at,
        updated_at
    FROM api_chats
    UNION ALL
    SELECT
        id,
        'Console'::inference_type AS inference_type,
        (SELECT model_id FROM prompts p WHERE p.id = prompt_id) AS model_id,
        (SELECT user_id FROM conversations c WHERE c.id = conversation_id) AS user_id,
        tokens_sent,
        tokens_received,
        time_taken_ms,
        created_at,
        updated_at
    FROM chats
),
recent_data AS (
    SELECT
        model_id,
        user_id,
        SUM(tokens_sent) AS tpm_sent,
        SUM(tokens_received) AS tpm_recv
    FROM combined_data
    WHERE created_at >= NOW() - INTERVAL '1 minute'
    GROUP BY model_id, user_id
)
SELECT
    model_id,
    user_id,
    tpm_sent,
    tpm_recv
FROM recent_data;

-- Grant permissions
GRANT SELECT ON inference_metrics TO bionic_application;
GRANT SELECT ON inference_metrics TO bionic_readonly;