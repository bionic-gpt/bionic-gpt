-- migrate:up
ALTER TABLE chats ADD COLUMN request_embeddings VECTOR;
ALTER TABLE chats ADD COLUMN response_embeddings VECTOR;

-- migrate:down
ALTER TABLE chats DROP COLUMN request_embeddings;
ALTER TABLE chats DROP COLUMN response_embeddings;

