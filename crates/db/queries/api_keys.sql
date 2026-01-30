--: ApiKey(prompt_id?, prompt_name?, prompt_type?, model_id?)

--! api_keys : ApiKey
SELECT
    a.id,
    a.name,
    a.prompt_id,
    a.user_id,
    a.team_id,
    (SELECT name FROM assistants.prompts p WHERE p.id = a.prompt_id) as prompt_name,
    (SELECT prompt_type FROM assistants.prompts p WHERE p.id = a.prompt_id) as prompt_type,
    (SELECT model_id FROM assistants.prompts p WHERE p.id = a.prompt_id) as model_id,
    a.api_key,
    a.created_at
FROM
    iam.api_keys a
WHERE 
    a.team_id = :team_id
AND
    a.user_id = current_app_user()
AND
    a.prompt_id IS NOT NULL
ORDER BY created_at DESC;

--! new_api_key
INSERT INTO iam.api_keys 
    (prompt_id, user_id, team_id, name, api_key)
VALUES
    (:prompt_id, :user_id, :team_id, :name, encode(digest(:api_key, 'sha256'), 'hex'));

--! new_mcp_api_key
INSERT INTO iam.api_keys
    (prompt_id, user_id, team_id, name, api_key)
VALUES
    (NULL, :user_id, :team_id, :name, encode(digest(:api_key, 'sha256'), 'hex'))
RETURNING id;

--! find_api_key : ApiKey
SELECT
    a.id,
    a.name,
    a.prompt_id,
    a.user_id,
    a.team_id,
    (SELECT name FROM assistants.prompts p WHERE p.id = a.prompt_id) as prompt_name,
    (SELECT prompt_type FROM assistants.prompts p WHERE p.id = a.prompt_id) as prompt_type,
    (SELECT model_id FROM assistants.prompts p WHERE p.id = a.prompt_id) as model_id,
    a.api_key,
    a.created_at
FROM
    iam.api_keys a
WHERE
    a.api_key = encode(digest(:api_key, 'sha256'), 'hex');

--! find_mcp_api_keys : ApiKey
SELECT
    a.id,
    a.name,
    a.prompt_id,
    a.user_id,
    a.team_id,
    (SELECT name FROM assistants.prompts p WHERE p.id = a.prompt_id) as prompt_name,
    (SELECT prompt_type FROM assistants.prompts p WHERE p.id = a.prompt_id) as prompt_type,
    (SELECT model_id FROM assistants.prompts p WHERE p.id = a.prompt_id) as model_id,
    a.api_key,
    a.created_at
FROM
    iam.api_keys a
WHERE
    a.team_id = :team_id
    AND a.prompt_id IS NULL
ORDER BY created_at DESC;

--! delete
DELETE FROM
    iam.api_keys
WHERE
    id = :api_key_id
AND
    team_id
    IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user());

--! new_api_chat
INSERT INTO llm.api_chats
    (api_key_id, content, role, status)
VALUES
    (:api_key_id, :content, :role, :status)
RETURNING id;
