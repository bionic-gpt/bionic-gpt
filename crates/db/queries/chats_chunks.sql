--: ChatChunks()

--! chunks_chats : ChatChunks
SELECT 
    chunk_id, 
    chat_id,
    (SELECT page_number from chunks c WHERE c.id = chunk_id) as page_number,
    (SELECT 
        file_name 
    FROM documents d 
    WHERE d.id = (SELECT document_id FROM chunks c WHERE c.id = chunk_id)) 
    AS file_name
FROM
    chunks_chats 
WHERE chat_id = :chat_id;

--! create_chunks_chats
INSERT INTO chunks_chats 
    (chunk_id, chat_id)
VALUES
    (:chunk_id, (SELECT id FROM chats WHERE conversation_id = :conversation_id ORDER BY created_at DESC LIMIT 1));