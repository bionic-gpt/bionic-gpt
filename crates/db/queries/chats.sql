--: Chat(content?, tool_calls?, tool_call_id?, tool_call_results?, attachments?)


--! new_chat(tool_call_id?)
INSERT INTO chats
    (conversation_id, prompt_id, tool_call_id, content, prompt, role)
VALUES
    (:conversation_id, :prompt_id, :tool_call_id, encrypt_text(:content), encrypt_text(:prompt), :role)
RETURNING id;
    
--! update_chat(tool_calls?, tool_call_results?)
UPDATE chats 
SET 
    content = encrypt_text(:content),
    tool_calls = encrypt_text(:tool_calls),
    tool_call_results = encrypt_text(:tool_call_results),
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
    decrypt_text(content) as content,
    role,
    tool_call_id,
    decrypt_text(tool_calls) as tool_calls,
    decrypt_text(tool_call_results) as tool_call_results,
    decrypt_text(prompt) as prompt,
    prompt_id,
    (SELECT name FROM models WHERE id IN (SELECT model_id FROM prompts WHERE id = prompt_id)) as model_name,
    status,
    (
        SELECT json_agg(json_build_object(
            'name', o.file_name,
            'type', o.mime_type,
            'size', o.file_size
        ))
        FROM chats_attachments ca
        JOIN objects o ON ca.object_id = o.id
        WHERE ca.chat_id = chats.id
    ) as attachments,
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
    decrypt_text(content) as content,
    role,
    tool_call_id,
    decrypt_text(tool_calls) as tool_calls,
    decrypt_text(tool_call_results) as tool_call_results,
    decrypt_text(prompt) as prompt,
    prompt_id,
    (SELECT name FROM models WHERE id IN (SELECT model_id FROM prompts WHERE id = prompt_id)) as model_name,
    status,
    (
        SELECT json_agg(json_build_object(
            'name', o.file_name,
            'type', o.mime_type,
            'size', o.file_size
        ))
        FROM chats_attachments ca
        JOIN objects o ON ca.object_id = o.id
        WHERE ca.chat_id = chats.id
    ) as attachments,
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
    decrypt_text(content) as content,
    role,
    tool_call_id,
    decrypt_text(tool_calls) as tool_calls,
    decrypt_text(tool_call_results) as tool_call_results,
    decrypt_text(prompt) as prompt,
    prompt_id,
    (SELECT name FROM models WHERE id IN (SELECT model_id FROM prompts WHERE id = prompt_id)) as model_name,
    status,
    (
        SELECT json_agg(json_build_object(
            'name', o.file_name,
            'type', o.mime_type,
            'size', o.file_size
        ))
        FROM chats_attachments ca
        JOIN objects o ON ca.object_id = o.id
        WHERE ca.chat_id = chats.id
    ) as attachments,
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

--! set_chat_status
UPDATE chats
SET
    status = :chat_status
WHERE
    id = :chat_id
AND
    -- Make sure the chat belongs to the user
    conversation_id IN (SELECT id FROM conversations WHERE user_id = current_app_user());