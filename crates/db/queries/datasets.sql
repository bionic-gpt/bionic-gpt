--! datasets : Dataset()
SELECT
    id,
    team_id, 
    name,
    chunking_strategy,
    visibility,
    combine_under_n_chars,
    new_after_n_chars,
    multipage_sections,
    (SELECT COUNT(id) FROM documents WHERE dataset_id = d.id) as count,
    created_at,
    updated_at
FROM 
    datasets d
WHERE
    team_id IN (SELECT team_id FROM team_users WHERE user_id = current_app_user())
ORDER BY updated_at;

--! dataset_by_pipeline_key : Dataset()
SELECT
    id,
    team_id, 
    name,
    chunking_strategy,
    visibility,
    combine_under_n_chars,
    new_after_n_chars,
    multipage_sections,
    (SELECT COUNT(id) FROM documents WHERE dataset_id = d.id) as count,
    created_at,
    updated_at
FROM 
    datasets d
WHERE
    d.id IN (
        SELECT dataset_id FROM document_pipelines WHERE api_key = :api_key
    ) ORDER BY updated_at;

--! dataset : Dataset()
SELECT
    id,
    team_id, 
    name,
    chunking_strategy,
    visibility,
    combine_under_n_chars,
    new_after_n_chars,
    multipage_sections,
    (SELECT COUNT(id) FROM documents WHERE dataset_id = d.id) as count,
    created_at,
    updated_at
FROM 
    datasets d
WHERE
    id = :dataset_id
AND
    team_id IN (
        SELECT 
            team_id 
        FROM team_users WHERE user_id = current_app_user())
ORDER BY updated_at;

--! pipeline_dataset : Dataset()
SELECT
    id,
    team_id, 
    name,
    chunking_strategy,
    visibility,
    combine_under_n_chars,
    new_after_n_chars,
    multipage_sections,
    (SELECT COUNT(id) FROM documents WHERE dataset_id = d.id) as count,
    created_at,
    updated_at
FROM 
    datasets d
WHERE
    id = :dataset_id
ORDER BY updated_at;

--! insert
INSERT INTO 
    datasets (
        team_id, 
        name,
        embeddings_model_id,
        chunking_strategy,
        combine_under_n_chars,
        new_after_n_chars,
        multipage_sections,
        visibility
    )
VALUES(
    :team_id, 
    :name,
    :embeddings_model_id,
    :chunking_strategy,
    :combine_under_n_chars,
    :new_after_n_chars,
    :multipage_sections,
    :visibility)
RETURNING id;