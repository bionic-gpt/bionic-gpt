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

--! create_chunks_chats
INSERT INTO chunks_chats 
    (chunk_id, chat_id)
VALUES
    (:chunk_id, (SELECT id FROM chats WHERE conversation_id = :conversation_id ORDER BY created_at DESC LIMIT 1));