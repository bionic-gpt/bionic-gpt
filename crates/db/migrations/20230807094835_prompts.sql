-- migrate:up


CREATE TABLE prompts (
    id SERIAL PRIMARY KEY, 
    model_id INT NOT NULL,
    name VARCHAR NOT NULL, 
    template VARCHAR NOT NULL, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

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