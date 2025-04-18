--: Chat(response?, function_call?, function_call_results?)


--! new_chat
INSERT INTO chats 
    (conversation_id, prompt_id, user_request, prompt)
VALUES
    (:conversation_id, :prompt_id, encrypt_text(:user_request), encrypt_text(:prompt))
RETURNING id;
    
--! update_chat(function_call?, function_call_results?)
UPDATE chats 
SET 
    response = encrypt_text(:response),
    function_call = encrypt_text(:function_call),
    function_call_results = encrypt_text(:function_call_results),
    tokens_received = :tokens_received,
    status = :chat_status
WHERE
    id = :chat_id
AND
    -- Make sure the chat belongs to the user
    conversation_id IN (SELECT id FROM conversations WHERE user_id = current_app_user());
    
--! update_prompt
UPDATE chats 
SET 
    prompt = encrypt_text(:prompt),
    tokens_sent = :tokens_sent
WHERE
    id = :chat_id
AND
    -- Make sure the chat belongs to the user
    conversation_id IN (SELECT id FROM conversations WHERE user_id = current_app_user());

--! chats : Chat
SELECT
    id,
    conversation_id,
    decrypt_text(user_request) as user_request,
    decrypt_text(function_call) as function_call,
    decrypt_text(function_call_results) as function_call_results,
    decrypt_text(prompt) as prompt,
    prompt_id,
    (SELECT name FROM models WHERE id IN (SELECT model_id FROM prompts WHERE id = prompt_id)) as model_name,
    decrypt_text(response) as response,
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
    decrypt_text(user_request) as user_request,
    decrypt_text(function_call) as function_call,
    decrypt_text(function_call_results) as function_call_results,
    decrypt_text(prompt) as prompt,
    prompt_id,
    (SELECT name FROM models WHERE id IN (SELECT model_id FROM prompts WHERE id = prompt_id)) as model_name,
    decrypt_text(response) as response,
    created_at,
    updated_at
FROM 
    chats
WHERE
    -- Make sure the chat belongs to the user
    conversation_id IN (SELECT id FROM conversations WHERE user_id = current_app_user())
AND 
    conversation_id = :conversation_id
ORDER BY updated_at ASC
LIMIT :limit;

--! chat : Chat
SELECT
    id,
    conversation_id,
    decrypt_text(user_request) as user_request,
    decrypt_text(function_call) as function_call,
    decrypt_text(function_call_results) as function_call_results,
    decrypt_text(prompt) as prompt,
    prompt_id,
    (SELECT name FROM models WHERE id IN (SELECT model_id FROM prompts WHERE id = prompt_id)) as model_name,
    decrypt_text(response) as response,
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