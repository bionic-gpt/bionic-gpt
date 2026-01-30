--: DatasetWithModel()

--! datasets : Dataset()
SELECT
    id,
    team_id, 
    external_id,
    name,
    chunking_strategy,
    visibility,
    combine_under_n_chars,
    new_after_n_chars,
    multipage_sections,
    (SELECT COUNT(id) FROM rag.documents WHERE dataset_id = d.id) as count,
    (SELECT name FROM model_registry.models WHERE id = d.embeddings_model_id) as embeddings_model_name,
    created_at,
    updated_at
FROM 
    rag.datasets d
WHERE
    is_project = false
AND
    (
        (visibility = 'Private' AND created_by = current_app_user()) 
        OR 
        (
            visibility = 'Team' 
            AND
            team_id IN (
                SELECT 
                    team_id 
                FROM tenancy.team_users WHERE user_id = current_app_user())
        )
        OR 
        (visibility = 'Company')
    )
ORDER BY updated_at;

--! dataset_by_external_id : DatasetWithModel
SELECT
    id,
    team_id,
    external_id,
    name,
    visibility,
    embeddings_model_id,
    chunking_strategy,
    combine_under_n_chars,
    new_after_n_chars,
    multipage_sections,
    created_by,
    created_at,
    updated_at
FROM
    rag.datasets
WHERE
    external_id = :external_id;

--! dataset_by_pipeline_key : Dataset()
SELECT
    id,
    team_id, 
    external_id,
    name,
    chunking_strategy,
    visibility,
    combine_under_n_chars,
    new_after_n_chars,
    multipage_sections,
    (SELECT COUNT(id) FROM rag.documents WHERE dataset_id = d.id) as count,
    (SELECT name FROM model_registry.models WHERE id = d.embeddings_model_id) as embeddings_model_name,
    created_at,
    updated_at
FROM 
    rag.datasets d
WHERE
    d.id IN (
        SELECT dataset_id FROM rag.document_pipelines WHERE api_key = :api_key
    ) ORDER BY updated_at;

--! dataset : Dataset()
SELECT
    id,
    team_id, 
    external_id,
    name,
    chunking_strategy,
    visibility,
    combine_under_n_chars,
    new_after_n_chars,
    multipage_sections,
    (SELECT COUNT(id) FROM rag.documents WHERE dataset_id = d.id) as count,
    (SELECT name FROM model_registry.models WHERE id = d.embeddings_model_id) as embeddings_model_name,
    created_at,
    updated_at
FROM 
    rag.datasets d
WHERE
    id = :dataset_id
AND

    (
        (visibility = 'Private' AND created_by = current_app_user()) 
        OR 
            (
                visibility = 'Team' 
                AND
                team_id IN (
                    SELECT 
                        team_id 
                    FROM tenancy.team_users WHERE user_id = current_app_user())
            )
        OR 
            (visibility = 'Company')
    )
ORDER BY updated_at;

--! pipeline_dataset : Dataset()
SELECT
    id,
    team_id, 
    external_id,
    name,
    chunking_strategy,
    visibility,
    combine_under_n_chars,
    new_after_n_chars,
    multipage_sections,
    (SELECT COUNT(id) FROM rag.documents WHERE dataset_id = d.id) as count,
    (SELECT name FROM model_registry.models WHERE id = d.embeddings_model_id) as embeddings_model_name,
    created_at,
    updated_at
FROM 
    rag.datasets d
WHERE
    id = :dataset_id
ORDER BY updated_at;

--! insert
INSERT INTO 
    rag.datasets (
        team_id, 
        name,
        embeddings_model_id,
        chunking_strategy,
        combine_under_n_chars,
        new_after_n_chars,
        multipage_sections,
        visibility,
        created_by
    )
VALUES(
    :team_id, 
    :name,
    :embeddings_model_id,
    :chunking_strategy,
    :combine_under_n_chars,
    :new_after_n_chars,
    :multipage_sections,
    :visibility,
    current_app_user())
RETURNING id;

--! insert_project
INSERT INTO 
    rag.datasets (
        team_id, 
        name,
        embeddings_model_id,
        chunking_strategy,
        combine_under_n_chars,
        new_after_n_chars,
        multipage_sections,
        visibility,
        is_project,
        created_by
    )
VALUES(
    :team_id, 
    :name,
    :embeddings_model_id,
    :chunking_strategy,
    :combine_under_n_chars,
    :new_after_n_chars,
    :multipage_sections,
    :visibility,
    true,
    current_app_user())
RETURNING id;

--! update
UPDATE 
    rag.datasets 
SET 
    name = :name, 
    visibility = :visibility,
    embeddings_model_id = :embeddings_model_id,
    chunking_strategy = :chunking_strategy,
    combine_under_n_chars = :combine_under_n_chars,
    new_after_n_chars = :new_after_n_chars,
    multipage_sections = :multipage_sections
WHERE
    id = :id
AND
    team_id
    IN (SELECT team_id FROM tenancy.team_users WHERE user_id = current_app_user());

--! delete
DELETE FROM
    rag.datasets
WHERE
    id = :id
AND
    team_id
    IN (SELECT team_id FROM tenancy.team_users WHERE user_id = current_app_user());
