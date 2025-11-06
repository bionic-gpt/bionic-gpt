--! unprocessed_documents : Chunk()
SELECT
    d.id,
    d.dataset_id,
    d.file_name,
    d.content
FROM
    documents d
WHERE
    failure_reason IS NULL
    AND
    id NOT IN (SELECT document_id FROM chunks WHERE document_id = d.id)
ORDER BY
    id
LIMIT :limit;

--! fail_document
UPDATE documents SET failure_reason = :failure_reason WHERE id = :id;
    
--! documents : Document(failure_reason?)
SELECT
    id,
    dataset_id, 
    file_name,
    failure_reason,
    (SELECT COUNT(id) FROM chunks WHERE document_id = d.id) as batches,
    content_size,
    (SELECT COUNT(id) FROM chunks WHERE document_id = d.id AND chunks IS NULL AND processed IS TRUE) as fail_count,
    (SELECT COUNT(id) FROM chunks WHERE document_id = d.id AND processed IS NOT TRUE) as waiting,
    created_at,
    updated_at
FROM 
    documents d
-- Ony dataset sthe user has access to.
WHERE
    dataset_id = :dataset_id
AND
    dataset_id 
    IN (SELECT id FROM datasets WHERE team_id
        IN (SELECT team_id FROM team_users WHERE user_id = current_app_user())
    )
ORDER BY updated_at;

--! document : Document(failure_reason?)
SELECT
    id,
    dataset_id, 
    file_name,
    failure_reason,
    (SELECT COUNT(id) FROM chunks WHERE document_id = d.id) as batches,
    content_size,
    (SELECT COUNT(id) FROM chunks WHERE document_id = d.id AND chunks IS NULL AND processed IS TRUE) as fail_count,
    (SELECT COUNT(id) FROM chunks WHERE document_id = d.id AND processed IS NOT TRUE) as waiting,
    created_at,
    updated_at
FROM 
    documents d
-- Ony document the user has access to.
WHERE
    d.id = :document_id
AND
    d.id 
    IN (SELECT id FROM documents WHERE dataset_id
        IN (SELECT id FROM datasets WHERE team_id
            IN (SELECT team_id FROM team_users WHERE user_id = current_app_user())
        )
    );

--: DatasetDocumentSummary(failure_reason?)

--! dataset_documents : DatasetDocumentSummary
SELECT
    d.id,
    d.dataset_id,
    d.file_name,
    d.content_size,
    d.created_at,
    d.updated_at,
    d.failure_reason,
    (
        SELECT COUNT(id)
        FROM chunks c
        WHERE c.document_id = d.id
    ) AS chunk_count
FROM
    documents d
WHERE
    d.dataset_id = :dataset_id
    AND EXISTS (
        SELECT 1
        FROM datasets ds
        WHERE ds.id = d.dataset_id
        AND ds.team_id = :team_id
    )
ORDER BY
    d.updated_at DESC
LIMIT :limit
OFFSET :offset;

--: DatasetDocumentDetail(failure_reason?)

--! dataset_document : DatasetDocumentDetail
SELECT
    d.id,
    d.dataset_id,
    d.file_name,
    d.content_size,
    d.created_at,
    d.updated_at,
    d.failure_reason,
    (
        SELECT COUNT(id)
        FROM chunks c
        WHERE c.document_id = d.id
    ) AS chunk_count
FROM
    documents d
WHERE
    d.id = :document_id
    AND d.dataset_id = :dataset_id
    AND EXISTS (
        SELECT 1
        FROM datasets ds
        WHERE ds.id = d.dataset_id
        AND ds.team_id = :team_id
    );

--! insert
INSERT INTO documents (
    dataset_id,
    file_name,
    content,
    content_size
) 
VALUES(:dataset_id, :file_name, :content, :content_size)
RETURNING id;

--! delete
DELETE FROM
    documents
WHERE
    id = :document_id
AND
    id
    IN (SELECT id FROM documents WHERE dataset_id
        IN (SELECT id FROM datasets WHERE team_id
            IN (SELECT team_id FROM team_users WHERE user_id = current_app_user())
        )
    );
