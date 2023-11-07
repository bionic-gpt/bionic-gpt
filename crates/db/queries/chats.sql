--: Chat(response?)


--! new_chat
INSERT INTO chats 
    (conversation_id, prompt_id, user_request, prompt)
VALUES
    (:conversation_id, :prompt_id, :user_request, :prompt);
    
--! update_chat
UPDATE chats 
SET 
    response = :response,
    status = :chat_status
WHERE
    id = :chat_id
AND
    -- Make sure the chat belongs to the user
    conversation_id IN (SELECT id FROM conversations WHERE user_id = current_app_user());
    
--! update_prompt
UPDATE chats 
SET 
    prompt = :prompt
WHERE
    id = :chat_id
AND
    -- Make sure the chat belongs to the user
    conversation_id IN (SELECT id FROM conversations WHERE user_id = current_app_user());

--! chats : Chat
SELECT
    id,
    conversation_id,
    user_request,
    prompt,
    prompt_id,
    (SELECT name FROM models WHERE id IN (SELECT model_id FROM prompts WHERE id = prompt_id)) as model_name,
    response,
    created_at,
    updated_at
FROM 
    chats
WHERE
    -- Make sure the chat belongs to the user
    conversation_id IN (SELECT id FROM conversations WHERE user_id = current_app_user())
AND 
    conversation_id = :conversation_id
ORDER BY updated_at;

--! chat_history : Chat
SELECT
    id,
    conversation_id,
    user_request,
    prompt,
    prompt_id,
    (SELECT name FROM models WHERE id IN (SELECT model_id FROM prompts WHERE id = prompt_id)) as model_name,
    response,
    created_at,
    updated_at
FROM 
    chats
WHERE
    -- Make sure the chat belongs to the user
    conversation_id IN (SELECT id FROM conversations WHERE user_id = current_app_user())
AND 
    conversation_id = :conversation_id
AND 
    status = 'Success'
ORDER BY updated_at DESC
LIMIT :limit;

--! chat : Chat
SELECT
    id,
    conversation_id,
    user_request,
    prompt,
    prompt_id,
    (SELECT name FROM models WHERE id IN (SELECT model_id FROM prompts WHERE id = prompt_id)) as model_name,
    response,
    created_at,
    updated_at
FROM 
    chats
WHERE
    -- Make sure the chat belongs to the user
    conversation_id IN (SELECT id FROM conversations WHERE user_id = current_app_user())
AND
    id = :chat_id
ORDER BY updated_at;