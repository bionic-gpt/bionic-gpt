--! unprocessed_chunks : Chunk()
SELECT
    id,
    text,
    (SELECT 
        base_url 
    FROM 
        models 
    WHERE 
        id IN (SELECT embeddings_model_id FROM datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM documents d WHERE d.id = document_id))
    ) as base_url,
    (SELECT 
        name 
    FROM 
        models 
    WHERE 
        id IN (SELECT embeddings_model_id FROM datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM documents d WHERE d.id = document_id))
    ) as model
FROM
    chunks
WHERE
    processed IS NOT TRUE;

--! delete
DELETE FROM chunks WHERE id = :embedding_id;