-- migrate:up
CREATE TYPE dataset_connection AS ENUM (
    'All', 
    'None', 
    'Selected'
);
COMMENT ON TYPE dataset_connection IS 'A prompt can use all datasets, no datasets or selected datasets.';

CREATE TABLE prompts (
    id SERIAL PRIMARY KEY, 
    organisation_id INT NOT NULL, 
    model_id INT NOT NULL,
    visibility visibility NOT NULL,
    dataset_connection dataset_connection NOT NULL,
    name VARCHAR NOT NULL, 
    max_history_items INT NOT NULL,
    max_chunks INT NOT NULL,
    max_tokens INT NOT NULL,
    temperature REAL CHECK (temperature >= 0 AND temperature <= 2),
    top_p REAL CHECK (top_p >= 0 AND top_p <= 1),
    system_prompt VARCHAR, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_organisation FOREIGN KEY(organisation_id)
        REFERENCES organisations(id) ON DELETE CASCADE,

    CONSTRAINT FK_model FOREIGN KEY(model_id)
        REFERENCES models(id) ON DELETE CASCADE
);

CREATE TABLE prompt_dataset (
    prompt_id INT NOT NULL, 
    dataset_id INT NOT NULL,

    CONSTRAINT FK_prompt FOREIGN KEY(prompt_id)
        REFERENCES prompts(id),

    CONSTRAINT FK_dataset FOREIGN KEY(dataset_id)
        REFERENCES datasets(id),

    UNIQUE(prompt_id, dataset_id)
);

-- Give access to the application user.
GRANT SELECT, INSERT, UPDATE, DELETE ON prompts TO ft_application;
GRANT USAGE, SELECT ON prompts_id_seq TO ft_application;
GRANT SELECT, INSERT, UPDATE, DELETE ON prompt_dataset TO ft_application;

-- Give access to the readonly user
GRANT SELECT ON prompts TO ft_readonly;
GRANT SELECT ON prompts_id_seq TO ft_readonly;
GRANT SELECT ON prompt_dataset TO ft_readonly;

-- migrate:down
DROP TABLE prompt_dataset;
DROP TABLE prompts;
DROP TYPE dataset_connection;