--: Model(api_key?)
--: ModelWithPrompt(api_key?, prompt_id?)

--! models : Model
SELECT
    id,
    name,
    model_type,
    base_url,
    api_key,
    tpm_limit,
    rpm_limit,
    context_size,
    created_at,
    updated_at
FROM 
    models
WHERE model_type = :model_type
ORDER BY updated_at;

--! all_models : ModelWithPrompt
SELECT DISTINCT
    m.id,
    m.name,
    m.model_type,
    m.base_url,
    m.api_key,
    m.tpm_limit,
    m.rpm_limit,
    m.context_size,
    m.created_at,
    m.updated_at,
    COALESCE(p.name, '') AS display_name,
    COALESCE(p.description, '') AS description,
    COALESCE(p.disclaimer, '') AS disclaimer,
    p.id AS prompt_id,
    COALESCE(p.example1, '') AS example1,
    COALESCE(p.example2, '') AS example2,
    COALESCE(p.example3, '') AS example3,
    COALESCE(p.example4, '') AS example4
FROM 
    models m
LEFT JOIN 
    prompts p ON m.id = p.model_id AND p.prompt_type = 'Model'
ORDER BY 
    m.updated_at;


--! get_system_model : Model
SELECT
    id,
    name,
    model_type,
    base_url,
    api_key,
    tpm_limit,
    rpm_limit,
    context_size,
    created_at,
    updated_at
FROM 
    models
WHERE
    model_type = 'LLM'
ORDER BY created_at
LIMIT 1;

--! get_system_embedding_model : Model
SELECT
    id,
    name,
    model_type,
    base_url,
    api_key,
    tpm_limit,
    rpm_limit,
    context_size,
    created_at,
    updated_at
FROM 
    models
WHERE
    model_type = 'Embeddings'
ORDER BY created_at
LIMIT 1;


--! model : Model
SELECT
    id,
    name,
    model_type,
    base_url,
    api_key,
    tpm_limit,
    rpm_limit,
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
    tpm_limit,
    rpm_limit,
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
    tpm_limit,
    rpm_limit,
    context_size
)
VALUES(
    :name, 
    :model_type,
    :base_url, 
    :api_key, 
    :tpm_limit,
    :rpm_limit,
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
    tpm_limit = :tpm_limit,
    rpm_limit = :rpm_limit,
    context_size = :context_size
WHERE
    id = :id;

--! delete
DELETE FROM
    models
WHERE
    id = :id;