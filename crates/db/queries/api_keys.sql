--: ApiKey(model_id?, model_name?)

--! api_keys : ApiKey
SELECT
    a.id,
    a.name,
    a.model_id,
    a.user_id,
    a.team_id,
    (SELECT display_name FROM model_registry.models m WHERE m.id = a.model_id) as model_name,
    a.api_key,
    a.created_at
FROM
    iam.api_keys a
WHERE 
    a.team_id = :team_id
AND
    a.user_id = current_app_user()
AND
    a.model_id IS NOT NULL
ORDER BY created_at DESC;

--! new_api_key
INSERT INTO iam.api_keys 
    (model_id, user_id, team_id, name, api_key)
VALUES
    (:model_id, :user_id, :team_id, :name, encode(digest(:api_key, 'sha256'), 'hex'));

--! new_mcp_api_key
INSERT INTO iam.api_keys
    (model_id, user_id, team_id, name, api_key)
VALUES
    (NULL, :user_id, :team_id, :name, encode(digest(:api_key, 'sha256'), 'hex'))
RETURNING id;

--! find_api_key : ApiKey
SELECT
    a.id,
    a.name,
    a.model_id,
    a.user_id,
    a.team_id,
    (SELECT display_name FROM model_registry.models m WHERE m.id = a.model_id) as model_name,
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
    a.model_id,
    a.user_id,
    a.team_id,
    (SELECT display_name FROM model_registry.models m WHERE m.id = a.model_id) as model_name,
    a.api_key,
    a.created_at
FROM
    iam.api_keys a
WHERE
    a.team_id = :team_id
    AND a.model_id IS NULL
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
