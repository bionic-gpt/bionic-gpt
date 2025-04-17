--! unprocessed_chunks : Chunk(api_key?)
SELECT
    id,
    decrypt_text(text) as text,
    (SELECT 
        base_url 
    FROM 
        models 
    WHERE 
        id IN (SELECT embeddings_model_id FROM datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM documents d WHERE d.id = document_id))
    ) as base_url,
    (SELECT 
        api_key 
    FROM 
        models 
    WHERE 
        id IN (SELECT embeddings_model_id FROM datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM documents d WHERE d.id = document_id))
    ) as api_key,
    (SELECT 
        name 
    FROM 
        models 
    WHERE 
        id IN (SELECT embeddings_model_id FROM datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM documents d WHERE d.id = document_id))
    ) as model,
    (SELECT 
        context_size 
    FROM 
        models 
    WHERE 
        id IN (SELECT embeddings_model_id FROM datasets ds WHERE ds.id IN
        (SELECT dataset_id FROM documents d WHERE d.id = document_id))
    ) as context_size
FROM
    chunks
WHERE
    processed IS NOT TRUE;

--! delete
DELETE FROM chunks WHERE id = :embedding_id;