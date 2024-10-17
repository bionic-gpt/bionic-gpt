--: Conversation()
--: History()

--! get_latest_conversation : Conversation
SELECT
    id,
    user_id,
    team_id,
    created_at
FROM 
    conversations
WHERE
    user_id = current_app_user()
AND
    -- Make sure the user has access to this conversation
    team_id IN (SELECT team_id FROM team_users WHERE user_id = current_app_user())
ORDER BY created_at DESC
LIMIT 1;

--! create_conversation(prompt_id?)
INSERT INTO conversations 
    (user_id, team_id, prompt_id)
VALUES
    (current_app_user(), :team_id, :prompt_id)
RETURNING id;

--! get_conversation_from_chat : Conversation
SELECT
    id,
    user_id,
    team_id,
    created_at
FROM 
    conversations
WHERE
    user_id = current_app_user()
AND 
    id IN (SELECT conversation_id FROM chats WHERE id = :chat_id);

--! history : History
WITH summary AS (
    SELECT * FROM chats
    WHERE id IN (SELECT MIN(id) FROM chats GROUP BY conversation_id)
)
SELECT 
    c.id, 
    CASE
        WHEN LENGTH(summary.user_request) > 150 THEN 
            LEFT(summary.user_request, 150) || '...'
        ELSE 
            summary.user_request
    END AS summary,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(c.created_at)::text) as created_at_iso,
    c.created_at
FROM 
    conversations c
JOIN 
    summary
ON 
    c.id = summary.conversation_id
AND
    c.user_id = current_app_user()
AND
    -- Make sure the user has access to this conversation
    c.team_id IN (
        SELECT team_id 
        FROM team_users 
        WHERE user_id = current_app_user()
    )
ORDER BY c.created_at DESC
LIMIT 100;

--! delete
DELETE FROM
    conversations
WHERE
    id = :id
AND
    user_id = current_app_user();