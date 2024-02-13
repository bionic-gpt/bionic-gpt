-- migrate:up


CREATE TABLE chunks_chats (
    chunk_id INT NOT NULL, 
    chat_id INT NOT NULL, 
    PRIMARY KEY (chunk_id, chat_id)
);

-- migrate:down
DROP TABLE chunks_chats;