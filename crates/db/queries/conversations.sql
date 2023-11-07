--: Conversation()
--: History()

--! get_latest_conversation : Conversation
SELECT
    id,
    user_id,
    organisation_id,
    created_at
FROM 
    conversations
WHERE
    user_id = current_app_user()
AND
    -- Make sure the user has access to this conversation
    organisation_id IN (SELECT organisation_id FROM organisation_users WHERE user_id = current_app_user())
ORDER BY created_at DESC
LIMIT 1;

--! create_conversation
INSERT INTO conversations 
    (user_id, organisation_id)
VALUES
    (current_app_user(), :organisation_id)
RETURNING id;

--! get_conversation_from_chat : Conversation
SELECT
    id,
    user_id,
    organisation_id,
    created_at
FROM 
    conversations
WHERE
    user_id = current_app_user()
AND 
    id IN (SELECT conversation_id FROM chats WHERE id = :chat_id);

--! history : History
SELECT
    id,
    created_at,
    (SELECT user_request FROM chats WHERE conversation_id = c.id ORDER BY created_at DESC LIMIT 1) as summary
FROM 
    conversations c
WHERE
    user_id = current_app_user()
AND
    -- Make sure the user has access to this conversation
    organisation_id IN (SELECT organisation_id FROM organisation_users WHERE user_id = current_app_user());
