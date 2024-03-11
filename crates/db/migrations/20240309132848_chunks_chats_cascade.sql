-- migrate:up
ALTER TABLE chunks_chats ADD CONSTRAINT fk_chunk FOREIGN KEY(chunk_id) REFERENCES chunks(id) ON DELETE CASCADE;
ALTER TABLE chunks_chats ADD CONSTRAINT fk_chat FOREIGN KEY(chat_id) REFERENCES chats(id) ON DELETE CASCADE;

-- migrate:down