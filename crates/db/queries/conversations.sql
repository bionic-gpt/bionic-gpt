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

--! create_conversation
INSERT INTO conversations 
    (user_id, team_id)
VALUES
    (current_app_user(), :team_id)
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
    summary.user_request as summary,
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
    );

--! delete
DELETE FROM
    conversations
WHERE
    id = :id
AND
    user_id = current_app_user();