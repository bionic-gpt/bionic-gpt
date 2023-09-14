--: Prompt()

--! prompts : Prompt
SELECT
    p.id,
    p.organisation_id, 
    p.name,
    p.template,
    (
        SELECT STRING_AGG(name, ', ') FROM datasets d WHERE d.id IN (
            SELECT dataset_id FROM prompt_dataset WHERE prompt_id = p.id
        )
    ) AS datasets,
    p.created_at,
    p.updated_at
FROM 
    prompts p
WHERE
    p.organisation_id IN (SELECT organisation_id FROM organisation_users WHERE user_id = current_app_user())
ORDER BY updated_at;

--! prompt : Prompt
SELECT
    p.id,
    p.organisation_id, 
    p.name,
    p.template,
    (
        SELECT STRING_AGG(name, ', ') FROM datasets d WHERE d.id IN (
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
    p.organisation_id IN (
        SELECT organisation_id 
        FROM organisation_users 
        WHERE user_id = current_app_user()
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
        SELECT prompt_id FROM prompts WHERE 
            organisation_id IN (
                SELECT organisation_id 
                FROM organisation_users 
                WHERE user_id = current_app_user()
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
    organisation_id, 
    name,
    template
)
VALUES(
    :organisation_id, :name, :template
)
RETURNING id;

--! update
UPDATE 
    prompts 
SET 
    organisation_id = :organisation_id, 
    name = :name, 
    template = :template
WHERE
    id = :id;