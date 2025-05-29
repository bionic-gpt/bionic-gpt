--: Prompt(image_icon_object_id?, temperature?, system_prompt?, api_key?, example1?, example2?, example3?, example4?)
--: SinglePrompt(temperature?, system_prompt?, embeddings_base_url?, embeddings_model?, api_key?, example1?, example2?, example3?, example4?)

--! update_image
UPDATE 
    prompts 
SET 
    image_icon_object_id = :image_icon_object_id
WHERE
    id = :prompt_id
AND
    created_by = current_app_user();

--! my_prompts : Prompt
SELECT
    p.id,
    (SELECT name FROM models WHERE id = p.model_id) as model_name, 
    (SELECT base_url FROM models WHERE id = p.model_id) as base_url, 
    (SELECT api_key FROM models WHERE id = p.model_id) as api_key, 
    (SELECT context_size FROM models WHERE id = p.model_id) as model_context_size, 
    (SELECT team_id FROM models WHERE id = p.model_id) as team_id, 
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
    -- Create a string showing the integrations connected to this prompt
    (
        SELECT
            COALESCE(STRING_AGG(pi.integration_id::text, ','), '')
        FROM
            prompt_integration pi
        WHERE
            pi.prompt_id = p.id
    )
    as selected_integrations,
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM integrations i WHERE i.id IN (
            SELECT integration_id FROM prompt_integration WHERE prompt_id = p.id
        )
    ) AS integrations,
    p.system_prompt,
    p.max_history_items,
    p.max_chunks,
    p.max_tokens,
    p.trim_ratio,
    p.temperature,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(p.created_at)::text) as created_at,
    trim(both '"' from to_json(p.updated_at)::text) as updated_at,
    p.created_by,
    COALESCE((SELECT CONCAT(u.first_name, ' ', u.last_name) FROM users u WHERE id = p.created_by), 
              (SELECT email FROM users WHERE id = p.created_by)) as author_name
FROM 
    prompts p
WHERE
    created_by = current_app_user()
AND 
    p.prompt_type = :prompt_type
ORDER BY updated_at;

--! prompts : Prompt
SELECT
    p.id,
    (SELECT name FROM models WHERE id = p.model_id) as model_name, 
    (SELECT base_url FROM models WHERE id = p.model_id) as base_url, 
    (SELECT api_key FROM models WHERE id = p.model_id) as api_key, 
    (SELECT context_size FROM models WHERE id = p.model_id) as model_context_size, 
    (SELECT team_id FROM models WHERE id = p.model_id) as team_id, 
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
    -- Create a string showing the integrations connected to this prompt
    (
        SELECT
            COALESCE(STRING_AGG(pi.integration_id::text, ','), '')
        FROM
            prompt_integration pi
        WHERE
            pi.prompt_id = p.id
    )
    as selected_integrations,
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM integrations i WHERE i.id IN (
            SELECT integration_id FROM prompt_integration WHERE prompt_id = p.id
        )
    ) AS integrations,
    p.system_prompt,
    p.max_history_items,
    p.max_chunks,
    p.max_tokens,
    p.trim_ratio,
    p.temperature,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(p.created_at)::text) as created_at,
    trim(both '"' from to_json(p.updated_at)::text) as updated_at,
    p.created_by,
    COALESCE(
        NULLIF((SELECT CONCAT(u.first_name, ' ', u.last_name) FROM users u WHERE id = p.created_by), ' '),
        (SELECT email FROM users WHERE id = p.created_by)
    ) as author_name
FROM 
    prompts p
WHERE
    (
        (
            p.visibility='Team' 
            AND 
            p.model_id IN (
                SELECT id FROM models WHERE team_id IN(
                    SELECT team_id 
                    FROM team_users 
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
    (SELECT name FROM models WHERE id = p.model_id) as model_name, 
    (SELECT base_url FROM models WHERE id = p.model_id) as base_url, 
    (SELECT api_key FROM models WHERE id = p.model_id) as api_key, 
    (SELECT context_size FROM models WHERE id = p.model_id) as model_context_size, 
    (SELECT team_id FROM models WHERE id = p.model_id) as team_id,  
    (SELECT base_url FROM models WHERE id IN 
        (SELECT embeddings_model_id FROM datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM prompt_dataset WHERE prompt_id = p.id LIMIT 1))) as embeddings_base_url, 
    (SELECT name FROM models WHERE id IN 
        (SELECT embeddings_model_id FROM datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM prompt_dataset WHERE prompt_id = p.id LIMIT 1))) as embeddings_model,
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
    -- Create a string showing the integrations connected to this prompt
    (
        SELECT
            COALESCE(STRING_AGG(pi.integration_id::text, ','), '')
        FROM
            prompt_integration pi
        WHERE
            pi.prompt_id = p.id
    )
    as selected_integrations,
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM integrations i WHERE i.id IN (
            SELECT integration_id FROM prompt_integration WHERE prompt_id = p.id
        )
    ) AS integrations,
    p.system_prompt,
    p.max_history_items,
    p.max_chunks,
    p.max_tokens,
    p.trim_ratio,
    p.temperature,
    p.prompt_type,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(p.created_at)::text) as created_at,
    trim(both '"' from to_json(p.updated_at)::text) as updated_at,
    p.created_by
FROM 
    prompts p
WHERE
    p.id = :prompts_id
AND
    (
        p.visibility='Team' 
        AND p.model_id IN (
        SELECT id FROM models WHERE team_id IN(
            SELECT team_id 
            FROM team_users 
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
    (SELECT name FROM models WHERE id = p.model_id) as model_name, 
    (SELECT base_url FROM models WHERE id = p.model_id) as base_url, 
    (SELECT api_key FROM models WHERE id = p.model_id) as api_key, 
    (SELECT context_size FROM models WHERE id = p.model_id) as model_context_size, 
    (SELECT team_id FROM models WHERE id = p.model_id) as team_id, 
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
    -- Create a string showing the integrations connected to this prompt
    (
        SELECT
            COALESCE(STRING_AGG(pi.integration_id::text, ','), '')
        FROM
            prompt_integration pi
        WHERE
            pi.prompt_id = p.id
    )
    as selected_integrations,
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM integrations i WHERE i.id IN (
            SELECT integration_id FROM prompt_integration WHERE prompt_id = p.id
        )
    ) AS integrations,
    p.system_prompt,
    p.max_history_items,
    p.max_chunks,
    p.max_tokens,
    p.trim_ratio,
    p.temperature,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(p.created_at)::text) as created_at,
    trim(both '"' from to_json(p.updated_at)::text) as updated_at,
    p.created_by,
    COALESCE(
        NULLIF((SELECT CONCAT(u.first_name, ' ', u.last_name) FROM users u WHERE id = p.created_by), ' '),
        (SELECT email FROM users WHERE id = p.created_by)
    ) as author_name
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
    (
        (d.visibility = 'Private' AND d.created_by = current_app_user()) 
        OR 
            (
                d.visibility = 'Team' 
                AND
                team_id IN (
                    SELECT 
                        team_id 
                    FROM team_users WHERE user_id = current_app_user())
            )
        OR 
            (d.visibility = 'Company')
    );

--! delete_prompt_datasets
DELETE FROM prompt_dataset
WHERE
    prompt_id = :prompts_id
AND
    prompt_id IN (
        SELECT id FROM prompts WHERE model_id IN(
            SELECT id FROM models WHERE team_id IN(
                SELECT team_id 
                FROM team_users 
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
    

--! insert(system_prompt?, example1?, example2?, example3?, example4?, image_icon_object_id?)
INSERT INTO prompts (
    team_id, 
    model_id, 
    category_id, 
    name,
    image_icon_object_id,
    visibility,
    system_prompt,
    max_history_items,
    max_chunks,
    max_tokens,
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
    :max_tokens,
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

--! update(system_prompt?, example1?, example2?, example3?, example4?)
UPDATE 
    prompts 
SET 
    model_id = :model_id, 
    category_id = :category_id, 
    name = :name, 
    visibility = :visibility,
    system_prompt = :system_prompt,
    max_history_items = :max_history_items,
    max_chunks = :max_chunks,
    max_tokens = :max_tokens,
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
        SELECT id FROM prompts WHERE model_id IN(
            SELECT id FROM models WHERE team_id IN(
                SELECT team_id 
                FROM team_users 
                WHERE user_id = current_app_user()
            )
        )
    )
AND 
    model_id IN (
        SELECT id FROM models WHERE team_id IN(
            SELECT team_id 
            FROM team_users 
            WHERE user_id = current_app_user()
        )
    );

--! delete
DELETE FROM
    prompts
WHERE
    id = :id
AND
    team_id
    IN (SELECT team_id FROM team_users WHERE user_id = current_app_user());

--! prompt_integrations : PromptIntegration()
SELECT
    i.id as integration_id,
    pi.prompt_id as prompt_id,
    i.name,
    i.integration_type,
    i.integration_status
FROM 
    integrations i
LEFT JOIN 
    prompt_integration pi
ON 
    i.id = pi.integration_id
WHERE
    pi.prompt_id = :prompts_id;

--! delete_prompt_integrations
DELETE FROM prompt_integration
WHERE
    prompt_id = :prompts_id
AND
    prompt_id IN (
        SELECT id FROM prompts WHERE model_id IN(
            SELECT id FROM models WHERE team_id IN(
                SELECT team_id 
                FROM team_users 
                WHERE user_id = current_app_user()
            )
        )
    );

--! insert_prompt_integration
INSERT INTO prompt_integration(
    prompt_id,
    integration_id
)
VALUES(
    :prompt_id, :integration_id);
