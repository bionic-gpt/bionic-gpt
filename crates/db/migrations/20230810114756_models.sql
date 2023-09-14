-- migrate:up


CREATE TABLE models (
    id SERIAL PRIMARY KEY, 
    name VARCHAR NOT NULL, 
    base_url VARCHAR NOT NULL, 
    template VARCHAR NOT NULL, 
    billion_parameters INT NOT NULL, 
    context_size_bytes INT NOT NULL, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO models 
    (name, base_url, template, billion_parameters, context_size_bytes) 
VALUES('ggml-gpt4all-j', 
'http://llm-api:8080',
'The prompt below is a question to answer, a task to complete, or a conversation to respond to; decide which and write an appropriate response.
### Prompt:
{{.Input}}
### Response:',
7,
2048);

-- Give access to the application user.
GRANT SELECT, INSERT, UPDATE, DELETE ON models TO ft_application;
GRANT USAGE, SELECT ON models_id_seq TO ft_application;

-- Give access to the readonly user
GRANT SELECT ON models TO ft_readonly;
GRANT SELECT ON models_id_seq TO ft_readonly;

-- migrate:down
DROP TABLE models;