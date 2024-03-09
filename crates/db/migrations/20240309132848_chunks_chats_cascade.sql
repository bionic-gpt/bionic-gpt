-- migrate:up
ALTER TABLE chunks_chats ADD CONSTRAINT FOREIGN KEY(chunk_id) REFERENCES chunks(id) ON DELETE CASCADE;
ALTER TABLE chunks_chats ADD CONSTRAINT FOREIGN KEY(chat_id) REFERENCES chats(id) ON DELETE CASCADE;

-- migrate:down