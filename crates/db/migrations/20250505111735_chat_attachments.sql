-- migrate:up
CREATE TABLE chats_attachments (
    object_id INT NOT NULL, 
    chat_id INT NOT NULL,
    FOREIGN KEY (object_id) REFERENCES objects(id) ON DELETE CASCADE,
    FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE
);

COMMENT ON TABLE chats_attachments IS 'Every chat can have a number of attachments';

-- Grant access
GRANT SELECT, INSERT ON chats_attachments TO bionic_application;
GRANT SELECT ON chats_attachments TO bionic_readonly;

-- migrate:down
DROP TABLE chats_attachments;
