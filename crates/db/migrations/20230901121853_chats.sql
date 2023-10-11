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
    user_id INT NOT NULL, 
    organisation_id INT NOT NULL, 
    status chat_status NOT NULL DEFAULT 'Pending',
    user_request VARCHAR NOT NULL, 
    prompt VARCHAR NOT NULL, 
    prompt_id INT NOT NULL, 
    response VARCHAR, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_prompt FOREIGN KEY(prompt_id)
        REFERENCES prompts(id) ON DELETE CASCADE,

    CONSTRAINT FK_organisation FOREIGN KEY(organisation_id)
        REFERENCES organisations(id) ON DELETE CASCADE,

    CONSTRAINT FK_user FOREIGN KEY(user_id)
        REFERENCES users(id) ON DELETE CASCADE
);

-- Give access to the application user.
GRANT SELECT, INSERT, UPDATE, DELETE ON chats TO ft_application;
GRANT USAGE, SELECT ON chats_id_seq TO ft_application;

-- Give access to the readonly user
GRANT SELECT ON chats TO ft_readonly;
GRANT SELECT ON chats_id_seq TO ft_readonly;

-- migrate:down

DROP TABLE chats;
DROP TYPE chat_status;

