--! documents : Document()
SELECT
    id,
    dataset_id, 
    file_name,
    (SELECT COUNT(id) FROM embeddings WHERE document_id = d.id) as batches,
    (SELECT SUM(LENGTH(text)) FROM embeddings WHERE document_id = d.id) as text_size,
    (SELECT COUNT(id) FROM embeddings WHERE document_id = d.id AND embeddings IS NULL AND processed IS TRUE) as fail_count,
    (SELECT COUNT(id) FROM embeddings WHERE document_id = d.id AND processed IS NOT TRUE) as waiting,
    created_at,
    updated_at
FROM 
    documents d
-- Ony dataset the user has access to.
WHERE
    dataset_id = :dataset_id
AND
    dataset_id 
    IN (SELECT id FROM datasets WHERE organisation_id
        IN (SELECT organisation_id FROM organisation_users WHERE user_id = current_app_user())
    )
ORDER BY updated_at;

--! insert
INSERT INTO documents (
    dataset_id,
    file_name
) 
VALUES(:dataset_id, :file_name)
RETURNING id;