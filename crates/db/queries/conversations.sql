--: Conversation()

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

--! delete
DELETE FROM
    conversations
WHERE
    id = :id
AND
    user_id = current_app_user();

--! count_attachments
SELECT 
    COUNT(ca.object_id) as count
FROM 
    conversations c
JOIN 
    chats ch ON c.id = ch.conversation_id
LEFT JOIN 
    chats_attachments ca ON ch.id = ca.chat_id
WHERE 
    c.id = :conversation_id
AND
    c.user_id = current_app_user();