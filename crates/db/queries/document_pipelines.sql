--: DocumentPipeline()

--! document_pipelines : DocumentPipeline
SELECT
    a.id,
    a.name,
    a.dataset_id,
    a.user_id,
    (SELECT name FROM datasets p WHERE p.id = a.dataset_id) as dataset_name,
    a.api_key,
    a.created_at
FROM
    document_pipelines a
WHERE 
    a.dataset_id IN (
        SELECT id FROM datasets WHERE organisation_id IN(
            SELECT organisation_id 
            FROM organisation_users 
            WHERE user_id = current_app_user()
            AND organisation_id = :organisation_id
        )
    )
ORDER BY created_at DESC;

--! insert
INSERT INTO document_pipelines 
    (dataset_id, user_id, name, api_key)
VALUES
    (:dataset_id, :user_id, :name, :api_key);

--! find_api_key : DocumentPipeline
SELECT
    a.id,
    a.name,
    a.dataset_id,
    a.user_id,
    (SELECT name FROM datasets p WHERE p.id = a.dataset_id) as dataset_name,
    a.api_key,
    a.created_at
FROM
    document_pipelines a
WHERE
    a.api_key = :api_key;
