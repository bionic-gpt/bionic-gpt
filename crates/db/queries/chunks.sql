--! unprocessed_chunks : Chunk(api_key?)
SELECT
    id,
    decrypt_text(text) as text,
    (SELECT 
        base_url 
    FROM 
        model_registry.models 
    WHERE 
        id IN (SELECT embeddings_model_id FROM rag.datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM rag.documents d WHERE d.id = document_id))
    ) as base_url,
    (SELECT 
        api_key 
    FROM 
        model_registry.models 
    WHERE 
        id IN (SELECT embeddings_model_id FROM rag.datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM rag.documents d WHERE d.id = document_id))
    ) as api_key,
    (SELECT 
        name 
    FROM 
        model_registry.models 
    WHERE 
        id IN (SELECT embeddings_model_id FROM rag.datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM rag.documents d WHERE d.id = document_id))
    ) as model,
    (SELECT 
        context_size 
    FROM 
        model_registry.models 
    WHERE 
        id IN (SELECT embeddings_model_id FROM rag.datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM rag.documents d WHERE d.id = document_id))
    ) as context_size
FROM
    rag.chunks
WHERE
    processed IS NOT TRUE
ORDER BY
    id
LIMIT :limit;

--: DocumentChunk()

--! document_chunks : DocumentChunk
SELECT
    c.id,
    c.document_id,
    c.page_number,
    decrypt_text(c.text) AS text
FROM
    rag.chunks c
    INNER JOIN rag.documents d ON d.id = c.document_id
    INNER JOIN rag.datasets ds ON ds.id = d.dataset_id
WHERE
    c.document_id = :document_id
    AND d.dataset_id = :dataset_id
    AND ds.team_id = :team_id
ORDER BY
    c.page_number ASC,
    c.id ASC
LIMIT :limit;

--! delete
DELETE FROM rag.chunks WHERE id = :embedding_id;
