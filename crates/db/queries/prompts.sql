--: Prompt(temperature?, max_completion_tokens?, system_prompt?, api_key?, example1?, example2?, example3?, example4?)
--: SinglePrompt(temperature?, max_completion_tokens?, system_prompt?, embeddings_base_url?, embeddings_model?, embeddings_api_key?, embeddings_context_size?, api_key?, example1?, example2?, example3?, example4?)

--! prompts : Prompt
SELECT
    p.id,
    (SELECT name FROM model_registry.models WHERE id = p.model_id) as model_name, 
    (SELECT base_url FROM model_registry.models WHERE id = p.model_id) as base_url, 
    (SELECT api_key FROM model_registry.models WHERE id = p.model_id) as api_key, 
    (SELECT context_size FROM model_registry.models WHERE id = p.model_id) as model_context_size, 
    (SELECT team_id FROM model_registry.models WHERE id = p.model_id) as team_id, 
    p.model_id,
    p.name,
    p.visibility,
    p.description,
    p.disclaimer,
    p.example1,
    p.example2,
    p.example3,
    p.example4,
    -- Creata a string showing the datsets connected to this prompt
    (
        SELECT 
            COALESCE(STRING_AGG(pd.dataset_id::text, ','), '')
        FROM 
            model_registry.prompt_dataset pd
        WHERE 
            pd.prompt_id = p.id
    ) 
    as selected_datasets, 
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM rag.datasets d WHERE d.id IN (
            SELECT dataset_id FROM model_registry.prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
    p.system_prompt,
    p.max_history_items,
    p.max_completion_tokens,
    p.trim_ratio,
    p.temperature,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(p.created_at)::text) as created_at,
    trim(both '"' from to_json(p.updated_at)::text) as updated_at,
    p.created_by,
    COALESCE(
        NULLIF((SELECT CONCAT(u.first_name, ' ', u.last_name) FROM iam.users u WHERE id = p.created_by), ' '),
        (SELECT email FROM iam.users WHERE id = p.created_by)
    ) as author_name
FROM 
    model_registry.prompts p
WHERE
    (
        (
            p.visibility='Team' 
            AND 
            p.model_id IN (
                SELECT id FROM model_registry.models WHERE team_id IN(
                    SELECT team_id 
                    FROM iam.team_users 
                    WHERE user_id = current_app_user()
                )
            AND 
            team_id = :team_id)
        )
        OR 
            (p.visibility='Company')
        OR 
            (p.visibility = 'Private' AND created_by = current_app_user()) 
    )
ORDER BY updated_at DESC;

--! prompt : SinglePrompt
SELECT
    p.id,
    (SELECT name FROM model_registry.models WHERE id = p.model_id) as model_name, 
    (SELECT base_url FROM model_registry.models WHERE id = p.model_id) as base_url, 
    (SELECT api_key FROM model_registry.models WHERE id = p.model_id) as api_key, 
    (SELECT context_size FROM model_registry.models WHERE id = p.model_id) as model_context_size, 
    (SELECT team_id FROM model_registry.models WHERE id = p.model_id) as team_id,  
    (SELECT base_url FROM model_registry.models WHERE id IN 
        (SELECT embeddings_model_id FROM rag.datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM model_registry.prompt_dataset WHERE prompt_id = p.id LIMIT 1))) as embeddings_base_url,
    (SELECT name FROM model_registry.models WHERE id IN 
        (SELECT embeddings_model_id FROM rag.datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM model_registry.prompt_dataset WHERE prompt_id = p.id LIMIT 1))) as embeddings_model,
    (SELECT api_key FROM model_registry.models WHERE id IN
        (SELECT embeddings_model_id FROM rag.datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM model_registry.prompt_dataset WHERE prompt_id = p.id LIMIT 1))) as embeddings_api_key,
    (SELECT context_size FROM model_registry.models WHERE id IN
        (SELECT embeddings_model_id FROM rag.datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM model_registry.prompt_dataset WHERE prompt_id = p.id LIMIT 1))) as embeddings_context_size,
    p.model_id,
    p.name,
    p.visibility,
    p.description,
    p.disclaimer,
    p.example1,
    p.example2,
    p.example3,
    p.example4,
    -- Creata a string showing the datsets connected to this prompt
    (
        SELECT 
            COALESCE(STRING_AGG(pd.dataset_id::text, ','), '')
        FROM 
            model_registry.prompt_dataset pd
        WHERE 
            pd.prompt_id = p.id
    ) 
    as selected_datasets, 
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM rag.datasets d WHERE d.id IN (
            SELECT dataset_id FROM model_registry.prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
    p.system_prompt,
    p.max_history_items,
    p.max_completion_tokens,
    p.trim_ratio,
    p.temperature,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(p.created_at)::text) as created_at,
    trim(both '"' from to_json(p.updated_at)::text) as updated_at,
    p.created_by
FROM 
    model_registry.prompts p
WHERE
    p.id = :prompts_id
AND
    (
        p.visibility='Team' 
        AND p.model_id IN (
        SELECT id FROM model_registry.models WHERE team_id IN(
            SELECT team_id 
            FROM iam.team_users 
            WHERE user_id = current_app_user()
        )
        AND team_id = :team_id
    )
    OR 
        (p.visibility='Company')
    OR 
        (p.visibility = 'Private' AND created_by = current_app_user()))
ORDER BY updated_at;

--! prompt_by_api_key : Prompt
SELECT
    p.id,
    (SELECT name FROM model_registry.models WHERE id = p.model_id) as model_name, 
    (SELECT base_url FROM model_registry.models WHERE id = p.model_id) as base_url, 
    (SELECT api_key FROM model_registry.models WHERE id = p.model_id) as api_key, 
    (SELECT context_size FROM model_registry.models WHERE id = p.model_id) as model_context_size, 
    (SELECT team_id FROM model_registry.models WHERE id = p.model_id) as team_id, 
    p.model_id,
    p.name,
    p.visibility,
    p.description,
    p.disclaimer,
    p.example1,
    p.example2,
    p.example3,
    p.example4,
    -- Creata a string showing the datsets connected to this prompt
    (
        SELECT 
            COALESCE(STRING_AGG(pd.dataset_id::text, ','), '')
        FROM 
            model_registry.prompt_dataset pd
        WHERE 
            pd.prompt_id = p.id
    ) 
    as selected_datasets, 
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM rag.datasets d WHERE d.id IN (
            SELECT dataset_id FROM model_registry.prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
    p.system_prompt,
    p.max_history_items,
    p.max_completion_tokens,
    p.trim_ratio,
    p.temperature,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(p.created_at)::text) as created_at,
    trim(both '"' from to_json(p.updated_at)::text) as updated_at,
    p.created_by,
    COALESCE(
        NULLIF((SELECT CONCAT(u.first_name, ' ', u.last_name) FROM iam.users u WHERE id = p.created_by), ' '),
        (SELECT email FROM iam.users WHERE id = p.created_by)
    ) as author_name
FROM 
    model_registry.prompts p
WHERE
    p.id IN (
        SELECT prompt_id FROM iam.api_keys WHERE api_key = encode(digest(:api_key, 'sha256'), 'hex')
    )
ORDER BY updated_at;

--! prompt_datasets : PromptDataset()
SELECT
    d.id as dataset_id,
    p.prompt_id as prompt_id,
    d.name
FROM 
    rag.datasets d
LEFT JOIN 
        model_registry.prompt_dataset p
    ON 
        d.id = p.dataset_id
WHERE
    p.prompt_id = :prompts_id
AND
    (
        (d.visibility = 'Private' AND d.created_by = current_app_user()) 
        OR 
            (
                d.visibility = 'Team' 
                AND
                team_id IN (
                    SELECT 
                        team_id 
                    FROM iam.team_users WHERE user_id = current_app_user())
            )
        OR 
            (d.visibility = 'Company')
    );

--! delete_prompt_datasets
DELETE FROM model_registry.prompt_dataset
WHERE
    prompt_id = :prompts_id
AND
    prompt_id IN (
        SELECT id FROM model_registry.prompts WHERE model_id IN(
            SELECT id FROM model_registry.models WHERE team_id IN(
                SELECT team_id 
                FROM iam.team_users 
                WHERE user_id = current_app_user()
            )
        )
    );

--! insert_prompt_dataset
INSERT INTO model_registry.prompt_dataset(
    prompt_id,
    dataset_id
)
VALUES(
    :prompt_id, :dataset_id
);
    

--! insert(system_prompt?, example1?, example2?, example3?, example4?, temperature?, max_completion_tokens?)
INSERT INTO model_registry.prompts (
    team_id, 
    model_id, 
    name,
    visibility,
    system_prompt,
    max_history_items,
    max_completion_tokens,
    trim_ratio,
    temperature,
    description,
    disclaimer,
    example1,
    example2,
    example3,
    example4,
    created_by
)
VALUES(
    :team_id, 
    :model_id,
    :name,
    :visibility,
    :system_prompt,
    :max_history_items,
    :max_completion_tokens,
    :trim_ratio,
    :temperature,
    :description,
    :disclaimer,
    :example1,
    :example2,
    :example3,
    :example4,
    current_app_user()
)
RETURNING id;

--! update(system_prompt?, example1?, example2?, example3?, example4?, temperature?, max_completion_tokens?)
UPDATE 
    model_registry.prompts
SET 
    model_id = :model_id, 
    name = :name, 
    visibility = :visibility,
    system_prompt = :system_prompt,
    max_history_items = :max_history_items,
    max_completion_tokens = :max_completion_tokens,
    trim_ratio = :trim_ratio,
    temperature = :temperature,
    description = :description,
    disclaimer = :disclaimer,
    example1 = :example1,
    example2 = :example2,
    example3 = :example3,
    example4 = :example4
WHERE
    id = :id
AND
    id IN (
        SELECT id FROM model_registry.prompts WHERE model_id IN(
            SELECT id FROM model_registry.models WHERE team_id IN(
                SELECT team_id 
                FROM iam.team_users 
                WHERE user_id = current_app_user()
            )
        )
    )
AND 
    model_id IN (
        SELECT id FROM model_registry.models WHERE team_id IN(
            SELECT team_id 
            FROM iam.team_users 
            WHERE user_id = current_app_user()
        )
    );

--! delete
DELETE FROM
    model_registry.prompts
WHERE
    id = :id
AND
    team_id
    IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user());
