--: Prompt(image_icon_object_id?, temperature?, max_completion_tokens?, system_prompt?, api_key?, example1?, example2?, example3?, example4?)
--: MyPrompt(image_icon_object_id?, api_key?)
--: SinglePrompt(temperature?, max_completion_tokens?, system_prompt?, embeddings_base_url?, embeddings_model?, embeddings_api_key?, embeddings_context_size?, api_key?, example1?, example2?, example3?, example4?)

--! update_image
UPDATE 
    prompting.prompts 
SET 
    image_icon_object_id = :image_icon_object_id
WHERE
    id = :prompt_id
AND
    created_by = current_app_user();

--! my_prompts : MyPrompt
SELECT
    p.id,
    (SELECT name FROM model_registry.models WHERE id = p.model_id) as model_name, 
    (SELECT base_url FROM model_registry.models WHERE id = p.model_id) as base_url, 
    (SELECT api_key FROM model_registry.models WHERE id = p.model_id) as api_key, 
    (SELECT context_size FROM model_registry.models WHERE id = p.model_id) as model_context_size, 
    (SELECT team_id FROM model_registry.models WHERE id = p.model_id) as team_id, 
    p.model_id,
    p.category_id,
    p.name,
    p.image_icon_object_id,
    p.visibility,
    p.description,
    (SELECT count(*) FROM prompting.prompt_dataset WHERE prompt_id = id) AS dataset_count,
    (SELECT count(*) FROM integrations.prompt_integration WHERE prompt_id = id) AS integration_count,
    (
        SELECT count(*) FROM automation.automation_cron_triggers WHERE prompt_id = id
    ) + (
        SELECT count(*) FROM automation.automation_webhook_triggers WHERE prompt_id = id
    ) AS trigger_count,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(p.created_at)::text) as created_at,
    trim(both '"' from to_json(p.updated_at)::text) as updated_at,
    p.created_by,
    COALESCE((SELECT CONCAT(u.first_name, ' ', u.last_name) FROM auth.users u WHERE id = p.created_by), 
              (SELECT email FROM auth.users WHERE id = p.created_by)) as author_name
FROM 
    prompting.prompts p
WHERE
    created_by = current_app_user()
AND 
    p.prompt_type = :prompt_type
ORDER BY updated_at;

--! prompts : Prompt
SELECT
    p.id,
    (SELECT name FROM model_registry.models WHERE id = p.model_id) as model_name, 
    (SELECT base_url FROM model_registry.models WHERE id = p.model_id) as base_url, 
    (SELECT api_key FROM model_registry.models WHERE id = p.model_id) as api_key, 
    (SELECT context_size FROM model_registry.models WHERE id = p.model_id) as model_context_size, 
    (SELECT team_id FROM model_registry.models WHERE id = p.model_id) as team_id, 
    p.model_id,
    p.category_id,
    p.name,
    p.image_icon_object_id,
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
            prompting.prompt_dataset pd
        WHERE 
            pd.prompt_id = p.id
    ) 
    as selected_datasets, 
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM rag.datasets d WHERE d.id IN (
            SELECT dataset_id FROM prompting.prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
    -- Create a string showing the integrations.integrations connected to this prompt
    (
        SELECT
            COALESCE(STRING_AGG(pi.integration_id::text, ','), '')
        FROM
            integrations.prompt_integration pi
        WHERE
            pi.prompt_id = p.id
    )
    as selected_integrations,
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM integrations.integrations i WHERE i.id IN (
            SELECT integration_id FROM integrations.prompt_integration WHERE prompt_id = p.id
        )
    ) AS integrations,
    p.system_prompt,
    p.max_history_items,
    p.max_chunks,
    p.max_completion_tokens,
    p.trim_ratio,
    p.temperature,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(p.created_at)::text) as created_at,
    trim(both '"' from to_json(p.updated_at)::text) as updated_at,
    p.created_by,
    COALESCE(
        NULLIF((SELECT CONCAT(u.first_name, ' ', u.last_name) FROM auth.users u WHERE id = p.created_by), ' '),
        (SELECT email FROM auth.users WHERE id = p.created_by)
    ) as author_name
FROM 
    prompting.prompts p
WHERE
    (
        (
            p.visibility='Team' 
            AND 
            p.model_id IN (
                SELECT id FROM model_registry.models WHERE team_id IN(
                    SELECT team_id 
                    FROM tenancy.team_users 
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
    AND p.prompt_type = :prompt_type
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
        (SELECT dataset_id FROM prompting.prompt_dataset WHERE prompt_id = p.id LIMIT 1))) as embeddings_base_url, 
    (SELECT name FROM model_registry.models WHERE id IN 
        (SELECT embeddings_model_id FROM rag.datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM prompting.prompt_dataset WHERE prompt_id = p.id LIMIT 1))) as embeddings_model,
    (SELECT api_key FROM model_registry.models WHERE id IN
        (SELECT embeddings_model_id FROM rag.datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM prompting.prompt_dataset WHERE prompt_id = p.id LIMIT 1))) as embeddings_api_key,
    (SELECT context_size FROM model_registry.models WHERE id IN
        (SELECT embeddings_model_id FROM rag.datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM prompting.prompt_dataset WHERE prompt_id = p.id LIMIT 1))) as embeddings_context_size,
    p.model_id,
    p.category_id,
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
            prompting.prompt_dataset pd
        WHERE 
            pd.prompt_id = p.id
    ) 
    as selected_datasets, 
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM rag.datasets d WHERE d.id IN (
            SELECT dataset_id FROM prompting.prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
    -- Create a string showing the integrations.integrations connected to this prompt
    (
        SELECT
            COALESCE(STRING_AGG(pi.integration_id::text, ','), '')
        FROM
            integrations.prompt_integration pi
        WHERE
            pi.prompt_id = p.id
    )
    as selected_integrations,
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM integrations.integrations i WHERE i.id IN (
            SELECT integration_id FROM integrations.prompt_integration WHERE prompt_id = p.id
        )
    ) AS integrations,
    p.system_prompt,
    p.max_history_items,
    p.max_chunks,
    p.max_completion_tokens,
    p.trim_ratio,
    p.temperature,
    p.prompt_type,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(p.created_at)::text) as created_at,
    trim(both '"' from to_json(p.updated_at)::text) as updated_at,
    p.created_by
FROM 
    prompting.prompts p
WHERE
    p.id = :prompts_id
AND
    (
        p.visibility='Team' 
        AND p.model_id IN (
        SELECT id FROM model_registry.models WHERE team_id IN(
            SELECT team_id 
            FROM tenancy.team_users 
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
    p.category_id,
    p.name,
    p.image_icon_object_id,
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
            prompting.prompt_dataset pd
        WHERE 
            pd.prompt_id = p.id
    ) 
    as selected_datasets, 
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM rag.datasets d WHERE d.id IN (
            SELECT dataset_id FROM prompting.prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
    -- Create a string showing the integrations.integrations connected to this prompt
    (
        SELECT
            COALESCE(STRING_AGG(pi.integration_id::text, ','), '')
        FROM
            integrations.prompt_integration pi
        WHERE
            pi.prompt_id = p.id
    )
    as selected_integrations,
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM integrations.integrations i WHERE i.id IN (
            SELECT integration_id FROM integrations.prompt_integration WHERE prompt_id = p.id
        )
    ) AS integrations,
    p.system_prompt,
    p.max_history_items,
    p.max_chunks,
    p.max_completion_tokens,
    p.trim_ratio,
    p.temperature,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(p.created_at)::text) as created_at,
    trim(both '"' from to_json(p.updated_at)::text) as updated_at,
    p.created_by,
    COALESCE(
        NULLIF((SELECT CONCAT(u.first_name, ' ', u.last_name) FROM auth.users u WHERE id = p.created_by), ' '),
        (SELECT email FROM auth.users WHERE id = p.created_by)
    ) as author_name
FROM 
    prompting.prompts p
WHERE
    p.id IN (
        SELECT prompt_id FROM auth.api_keys WHERE api_key = encode(digest(:api_key, 'sha256'), 'hex')
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
        prompting.prompt_dataset p
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
                    FROM tenancy.team_users WHERE user_id = current_app_user())
            )
        OR 
            (d.visibility = 'Company')
    );

--! delete_prompt_datasets
DELETE FROM prompting.prompt_dataset
WHERE
    prompt_id = :prompts_id
AND
    prompt_id IN (
        SELECT id FROM prompting.prompts WHERE model_id IN(
            SELECT id FROM model_registry.models WHERE team_id IN(
                SELECT team_id 
                FROM tenancy.team_users 
                WHERE user_id = current_app_user()
            )
        )
    );

--! insert_prompt_dataset
INSERT INTO prompting.prompt_dataset(
    prompt_id,
    dataset_id
)
VALUES(
    :prompt_id, :dataset_id
);
    

--! insert(system_prompt?, example1?, example2?, example3?, example4?, image_icon_object_id?, temperature?, max_completion_tokens?)
INSERT INTO prompting.prompts (
    team_id, 
    model_id, 
    category_id, 
    name,
    image_icon_object_id,
    visibility,
    system_prompt,
    max_history_items,
    max_chunks,
    max_completion_tokens,
    trim_ratio,
    temperature,
    description,
    disclaimer,
    example1,
    example2,
    example3,
    example4,
    prompt_type,
    created_by
)
VALUES(
    :team_id, 
    :model_id,
    :category_id, 
    :name,
    :image_icon_object_id,
    :visibility,
    :system_prompt,
    :max_history_items,
    :max_chunks,
    :max_completion_tokens,
    :trim_ratio,
    :temperature,
    :description,
    :disclaimer,
    :example1,
    :example2,
    :example3,
    :example4,
    :prompt_type,
    current_app_user()
)
RETURNING id;

--! update(system_prompt?, example1?, example2?, example3?, example4?, temperature?, max_completion_tokens?)
UPDATE 
    prompting.prompts 
SET 
    model_id = :model_id, 
    category_id = :category_id, 
    name = :name, 
    visibility = :visibility,
    system_prompt = :system_prompt,
    max_history_items = :max_history_items,
    max_chunks = :max_chunks,
    max_completion_tokens = :max_completion_tokens,
    trim_ratio = :trim_ratio,
    temperature = :temperature,
    description = :description,
    disclaimer = :disclaimer,
    example1 = :example1,
    example2 = :example2,
    example3 = :example3,
    example4 = :example4,
    prompt_type = :prompt_type
WHERE
    id = :id
AND
    id IN (
        SELECT id FROM prompting.prompts WHERE model_id IN(
            SELECT id FROM model_registry.models WHERE team_id IN(
                SELECT team_id 
                FROM tenancy.team_users 
                WHERE user_id = current_app_user()
            )
        )
    )
AND 
    model_id IN (
        SELECT id FROM model_registry.models WHERE team_id IN(
            SELECT team_id 
            FROM tenancy.team_users 
            WHERE user_id = current_app_user()
        )
    );

--! delete
DELETE FROM
    prompting.prompts
WHERE
    id = :id
AND
    team_id
    IN (SELECT team_id FROM tenancy.team_users WHERE user_id = current_app_user());
