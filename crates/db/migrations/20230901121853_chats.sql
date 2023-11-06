-- migrate:up
CREATE TYPE chat_status AS ENUM (
    'Pending', 
    'Success', 
    'Cancelled', 
    'Error'
);
COMMENT ON TYPE chat_status IS 'The status of this part of the conversation with the AI';

CREATE TABLE chats (
    id SERIAL PRIMARY KEY, 
    conversation_id BIGINT NOT NULL, 
    status chat_status NOT NULL DEFAULT 'Pending',
    user_request VARCHAR NOT NULL, 
    prompt VARCHAR NOT NULL, 
    prompt_id INT NOT NULL, 
    response VARCHAR, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_prompt FOREIGN KEY(prompt_id)
        REFERENCES prompts(id) ON DELETE CASCADE,

    CONSTRAINT FK_conversation FOREIGN KEY(conversation_id)
        REFERENCES conversations(id) ON DELETE CASCADE
);
COMMENT ON TABLE chats IS 'Questions from the user and the response from the LLM';

CREATE TABLE conversations (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INT NOT NULL, 
    organisation_id INT NOT NULL, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_organisation FOREIGN KEY(organisation_id)
        REFERENCES organisations(id) ON DELETE CASCADE,

    CONSTRAINT FK_user FOREIGN KEY(user_id)
        REFERENCES users(id) ON DELETE CASCADE
);
COMMENT ON TABLE conversations IS 'Collect together the users chats a bit like a history';


-- Give access to the application user.
GRANT SELECT, INSERT, UPDATE, DELETE ON chats TO ft_application;
GRANT USAGE, SELECT ON chats_id_seq TO ft_application;

-- Give access to the readonly user
GRANT SELECT ON chats TO ft_readonly;
GRANT SELECT ON chats_id_seq TO ft_readonly;

-- migrate:down

DROP TABLE chats;
DROP TABLE conversations;
DROP TYPE chat_status;

