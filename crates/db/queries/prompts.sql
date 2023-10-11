--: Prompt(temperature?, top_p?, system_prompt?)

--! prompts : Prompt
SELECT
    p.id,
    (SELECT name FROM models WHERE id = p.model_id) as model_name, 
    (SELECT base_url FROM models WHERE id = p.model_id) as base_url, 
    (SELECT context_size FROM models WHERE id = p.model_id) as model_context_size, 
    (SELECT organisation_id FROM models WHERE id = p.model_id) as organisation_id, 
    p.model_id,
    p.name,
    p.visibility,
    p.dataset_connection,
    -- Creata a string showing the datsets connected to this prompt
    (
        SELECT 
            COALESCE(STRING_AGG(pd.dataset_id::text, ','), '')
        FROM 
            prompt_dataset pd
        WHERE 
            pd.prompt_id = p.id
    ) 
    as selected_datasets, 
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM datasets d WHERE d.id IN (
            SELECT dataset_id FROM prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
    p.system_prompt,
    p.max_history_items,
    p.max_chunks,
    p.max_tokens,
    p.temperature,
    p.top_p,
    p.created_at,
    p.updated_at
FROM 
    prompts p
WHERE
    p.model_id IN (
        SELECT id FROM models WHERE organisation_id IN(
            SELECT organisation_id 
            FROM organisation_users 
            WHERE user_id = current_app_user()
        )
        AND organisation_id = :organisation_id
    )
ORDER BY updated_at;

--! prompt : Prompt
SELECT
    p.id,
    (SELECT name FROM models WHERE id = p.model_id) as model_name, 
    (SELECT base_url FROM models WHERE id = p.model_id) as base_url, 
    (SELECT context_size FROM models WHERE id = p.model_id) as model_context_size, 
    (SELECT organisation_id FROM models WHERE id = p.model_id) as organisation_id, 
    p.model_id,
    p.name,
    p.visibility,
    p.dataset_connection,
    -- Creata a string showing the datsets connected to this prompt
    (
        SELECT 
            COALESCE(STRING_AGG(pd.dataset_id::text, ','), '')
        FROM 
            prompt_dataset pd
        WHERE 
            pd.prompt_id = p.id
    ) 
    as selected_datasets, 
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM datasets d WHERE d.id IN (
            SELECT dataset_id FROM prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
    p.system_prompt,
    p.max_history_items,
    p.max_chunks,
    p.max_tokens,
    p.temperature,
    p.top_p,
    p.created_at,
    p.updated_at
FROM 
    prompts p
WHERE
    p.id = :prompts_id
AND
    p.model_id IN (
        SELECT id FROM models WHERE organisation_id IN(
            SELECT organisation_id 
            FROM organisation_users 
            WHERE user_id = current_app_user()
        )
        AND organisation_id = :organisation_id
    )
ORDER BY updated_at;

--! prompt_by_api_key : Prompt
SELECT
    p.id,
    (SELECT name FROM models WHERE id = p.model_id) as model_name, 
    (SELECT base_url FROM models WHERE id = p.model_id) as base_url, 
    (SELECT context_size FROM models WHERE id = p.model_id) as model_context_size, 
    (SELECT organisation_id FROM models WHERE id = p.model_id) as organisation_id, 
    p.model_id,
    p.name,
    p.visibility,
    p.dataset_connection,
    -- Creata a string showing the datsets connected to this prompt
    (
        SELECT 
            COALESCE(STRING_AGG(pd.dataset_id::text, ','), '')
        FROM 
            prompt_dataset pd
        WHERE 
            pd.prompt_id = p.id
    ) 
    as selected_datasets, 
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM datasets d WHERE d.id IN (
            SELECT dataset_id FROM prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
    p.system_prompt,
    p.max_history_items,
    p.max_chunks,
    p.max_tokens,
    p.temperature,
    p.top_p,
    p.created_at,
    p.updated_at
FROM 
    prompts p
WHERE
    p.id IN (
        SELECT prompt_id FROM api_keys WHERE api_key = :api_key
    )
ORDER BY updated_at;

--! prompt_datasets : PromptDataset()
SELECT
    d.id as dataset_id,
    p.prompt_id as prompt_id,
    d.name
FROM 
    datasets d
LEFT JOIN 
        prompt_dataset p
    ON 
        d.id = p.dataset_id
WHERE
    p.prompt_id = :prompts_id
AND
    d.organisation_id IN (
        SELECT organisation_id 
        FROM organisation_users 
        WHERE user_id = current_app_user()
    );

--! delete_prompt_datasets
DELETE FROM prompt_dataset
WHERE
    prompt_id = :prompts_id
AND
    prompt_id IN (
        SELECT id FROM prompts WHERE model_id IN(
            SELECT id FROM models WHERE organisation_id IN(
                SELECT organisation_id 
                FROM organisation_users 
                WHERE user_id = current_app_user()
            )
        )
    );

--! insert_prompt_dataset
INSERT INTO prompt_dataset(
    prompt_id,
    dataset_id
)
VALUES(
    :prompt_id, :dataset_id
);
    

--! insert(system_prompt?)
INSERT INTO prompts (
    organisation_id, 
    model_id, 
    name,
    visibility,
    dataset_connection,
    system_prompt,
    max_history_items,
    max_chunks,
    max_tokens,
    temperature,
    top_p
)
VALUES(
    :organisation_id, 
    :model_id,
    :name,
    :visibility,
    :dataset_connection,
    :system_prompt,
    :max_history_items,
    :max_chunks,
    :max_tokens,
    :temperature,
    :top_p
)
RETURNING id;

--! update(system_prompt?)
UPDATE 
    prompts 
SET 
    model_id = :model_id, 
    name = :name, 
    visibility = :visibility,
    dataset_connection = :dataset_connection,
    system_prompt = :system_prompt,
    max_history_items = :max_history_items,
    max_chunks = :max_chunks,
    max_tokens = :max_tokens,
    temperature = :temperature,
    top_p = :top_p
WHERE
    id = :id
AND
    id IN (
        SELECT id FROM prompts WHERE model_id IN(
            SELECT id FROM models WHERE organisation_id IN(
                SELECT organisation_id 
                FROM organisation_users 
                WHERE user_id = current_app_user()
            )
        )
    )
AND 
    model_id IN (
        SELECT id FROM models WHERE organisation_id IN(
            SELECT organisation_id 
            FROM organisation_users 
            WHERE user_id = current_app_user()
        )
    );