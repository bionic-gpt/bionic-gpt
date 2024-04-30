--: ApiKey()

--! api_keys : ApiKey
SELECT
    a.id,
    a.name,
    a.prompt_id,
    a.user_id,
    (SELECT name FROM prompts p WHERE p.id = a.prompt_id) as prompt_name,
    a.api_key,
    a.created_at
FROM
    api_keys a
WHERE 
    a.team_id = :team_id
AND
    a.user_id = current_app_user()
ORDER BY created_at DESC;

--! new_api_key
INSERT INTO api_keys 
    (prompt_id, user_id, team_id, name, api_key)
VALUES
    (:prompt_id, :user_id, :team_id, :name, :api_key);

--! find_api_key : ApiKey
SELECT
    a.id,
    a.name,
    a.prompt_id,
    a.user_id,
    (SELECT name FROM prompts p WHERE p.id = a.prompt_id) as prompt_name,
    a.api_key,
    a.created_at
FROM
    api_keys a
WHERE
    a.api_key = :api_key;

--! delete
DELETE FROM
    api_keys
WHERE
    id = :api_key_id
AND
    team_id
    IN (SELECT team_id FROM team_users WHERE user_id = current_app_user());

--! new_api_chat
INSERT INTO api_chats 
    (api_key_id, prompt, tokens_sent)
VALUES
    (:api_key_id, :prompt, :tokens_sent)
RETURNING id;
     
--! update_api_chat
UPDATE api_chats 
SET 
    response = :response,
    status = :chat_status,
    tokens_received = :tokens_received
WHERE
    api_key_id IN (SELECT id FROM api_keys WHERE api_key = :api_key)
AND id = :api_chat_id;
   