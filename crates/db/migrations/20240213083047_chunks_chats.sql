-- migrate:up


CREATE TABLE chunks_chats (
    chunk_id INT NOT NULL, 
    chat_id INT NOT NULL
);

COMMENT ON TABLE chunks_chats IS 'For each chat, track the chunks used as part of the prompt.';


-- Grant access
GRANT SELECT, INSERT ON chunks_chats TO bionic_application;

GRANT SELECT ON chunks_chats TO bionic_readonly;

-- migrate:down
DROP TABLE chunks_chats;