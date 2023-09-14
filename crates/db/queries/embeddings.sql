--! unprocessed_embeddings : Embedding()
SELECT
    id,
    text
FROM
    embeddings
WHERE
    processed IS NOT TRUE;

--! delete
DELETE FROM embeddings WHERE id = :embedding_id;