--: Prompt()

--! prompts : Prompt
SELECT
    p.id,
    (SELECT name FROM models WHERE id = p.model_id) as model_name, 
    p.name,
    p.dataset_connection,
    p.template,
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM datasets d WHERE d.id IN (
            SELECT dataset_id FROM prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
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
    p.name,
    p.dataset_connection,
    p.template,
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM datasets d WHERE d.id IN (
            SELECT dataset_id FROM prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
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
    p.name,
    p.dataset_connection,
    p.template,
    (
        SELECT COALESCE(STRING_AGG(name, ', '), '') FROM datasets d WHERE d.id IN (
            SELECT dataset_id FROM prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
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
    

--! insert
INSERT INTO prompts (
    model_id, 
    name,
    dataset_connection,
    template
)
VALUES(
    :model_id, :name, :dataset_connection, :template
)
RETURNING id;

--! update
UPDATE 
    prompts 
SET 
    model_id = :model_id, 
    name = :name, 
    dataset_connection = :dataset_connection,
    template = :template
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