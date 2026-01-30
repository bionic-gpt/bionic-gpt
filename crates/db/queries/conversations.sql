--: Conversation()

--! set_pending_to_success
UPDATE
    llm.chats
SET 
    status = 'Success'
WHERE
    status = 'Pending'
AND
    conversation_id = :id;

--! get_latest_conversation : Conversation
SELECT
    id,
    user_id,
    team_id,
    created_at
FROM 
    llm.conversations
WHERE
    user_id = current_app_user()
AND
    -- Make sure the user has access to this conversation
    team_id IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user())
ORDER BY created_at DESC
LIMIT 1;

--! create_conversation
INSERT INTO llm.conversations 
    (user_id, team_id)
VALUES
    (current_app_user(), :team_id)
RETURNING id;

--! create_project_conversation
INSERT INTO llm.conversations 
    (user_id, team_id, project_id)
VALUES
    (current_app_user(), :team_id, :project_id)
RETURNING id;

--! get_conversation_from_chat : Conversation
SELECT
    id,
    user_id,
    team_id,
    created_at
FROM 
    llm.conversations
WHERE
    user_id = current_app_user()
AND 
    id IN (SELECT conversation_id FROM llm.chats WHERE id = :chat_id);

--! delete
DELETE FROM
    llm.conversations
WHERE
    id = :id
AND
    user_id = current_app_user();

--! count_attachments
SELECT 
    COUNT(ca.object_id) as count
FROM 
    llm.conversations c
JOIN 
    llm.chats ch ON c.id = ch.conversation_id
LEFT JOIN 
    llm.chats_attachments ca ON ch.id = ca.chat_id
WHERE 
    c.id = :conversation_id
AND
    c.user_id = current_app_user();

--: ConversationContextSize()
--! conversation_context_size : ConversationContextSize
SELECT
    m.context_size
FROM
    llm.conversations c
JOIN llm.chats ch ON c.id = ch.conversation_id
    AND ch.id = (
        SELECT MAX(id)
        FROM llm.chats
        WHERE conversation_id = :conversation_id
    )
JOIN assistants.prompts p ON ch.prompt_id = p.id
JOIN model_registry.models m ON p.model_id = m.id
WHERE c.id = :conversation_id
  AND c.user_id = current_app_user();
