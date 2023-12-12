--: Model(api_key?)

--! models : Model
SELECT
    id,
    name,
    model_type,
    base_url,
    api_key,
    billion_parameters,
    context_size,
    created_at,
    updated_at
FROM 
    models
WHERE model_type = :model_type
ORDER BY updated_at;

--! get_system_model : Model
SELECT
    id,
    name,
    model_type,
    base_url,
    api_key,
    billion_parameters,
    context_size,
    created_at,
    updated_at
FROM 
    models
WHERE
    model_type = 'LLM'
ORDER BY created_at
LIMIT 1;


--! model : Model
SELECT
    id,
    name,
    model_type,
    base_url,
    api_key,
    billion_parameters,
    context_size,
    created_at,
    updated_at
FROM 
    models
WHERE
    id = :model_id
ORDER BY updated_at;

--! model_host_by_chat_id : Model
SELECT
    id,
    name,
    model_type,
    base_url,
    api_key,
    billion_parameters,
    context_size,
    created_at,
    updated_at
FROM 
    models
WHERE
    id IN (SELECT model_id FROM prompts p WHERE p.id IN (
        SELECT prompt_id FROM chats WHERE id = :chat_id
    ))
ORDER BY updated_at;


--! insert(api_key?)
INSERT INTO models (
    name,
    model_type,
    base_url,
    api_key,
    billion_parameters,
    context_size
)
VALUES(
    :name, 
    :model_type,
    :base_url, 
    :api_key, 
    :billion_parameters, 
    :context_size
)
RETURNING id;

--! update(api_key?)
UPDATE 
    models 
SET 
    name = :name,
    model_type = :model_type,
    base_url = :base_url,
    api_key = :api_key,
    billion_parameters = :billion_parameters,
    context_size = :context_size
WHERE
    id = :id;

--! delete
DELETE FROM
    models
WHERE
    id = :id;