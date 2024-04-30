-- migrate:up
ALTER TABLE chats ADD COLUMN tokens_sent INT NOT NULL DEFAULT 0;
ALTER TABLE chats ADD COLUMN tokens_received INT NOT NULL DEFAULT 0;
ALTER TABLE chats ADD COLUMN time_taken_ms INT NOT NULL DEFAULT 0;

-- migrate:down
ALTER TABLE chats DROP COLUMN tokens_sent;
ALTER TABLE chats DROP COLUMN tokens_received;
ALTER TABLE chats DROP COLUMN time_taken_ms;
