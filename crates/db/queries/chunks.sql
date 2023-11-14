--! unprocessed_chunks : Chunk()
SELECT
    id,
    text
FROM
    chunks
WHERE
    processed IS NOT TRUE;

--! delete
DELETE FROM chunks WHERE id = :embedding_id;