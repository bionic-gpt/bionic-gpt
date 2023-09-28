--: Model()

--! models : Model
SELECT
    id,
    name,
    base_url,
    billion_parameters,
    context_size_bytes,
    created_at,
    updated_at
FROM 
    models
WHERE
    organisation_id IN (SELECT organisation_id FROM organisation_users WHERE user_id = current_app_user())
ORDER BY updated_at;

--! model : Model
SELECT
    id,
    name,
    base_url,
    billion_parameters,
    context_size_bytes,
    created_at,
    updated_at
FROM 
    models
WHERE
    id = :model_id
ORDER BY updated_at;

--! model_host_by_chat_id
SELECT
    base_url
FROM 
    models
WHERE
    id IN (SELECT model_id FROM prompts p WHERE p.id IN (
        SELECT prompt_id FROM chats WHERE id = :chat_id
    ))
AND
    organisation_id IN (SELECT organisation_id FROM organisation_users WHERE user_id = current_app_user())
ORDER BY updated_at;


--! insert
INSERT INTO models (
    name,
    organisation_id,
    base_url,
    billion_parameters,
    context_size_bytes
)
VALUES(
    :name, :organisation_id, :base_url, :billion_parameters, :context_size_bytes
)
RETURNING id;

--! update
UPDATE 
    models 
SET 
    name = :name, 
    base_url = :base_url,
    billion_parameters = :billion_parameters,
    context_size_bytes = :context_size_bytes
WHERE
    id = :id;