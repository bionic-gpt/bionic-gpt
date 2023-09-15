-- migrate:up


CREATE TABLE models (
    id SERIAL PRIMARY KEY, 
    organisation_id INT NOT NULL, 
    name VARCHAR NOT NULL, 
    base_url VARCHAR NOT NULL, 
    billion_parameters INT NOT NULL, 
    context_size_bytes INT NOT NULL, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_organisation FOREIGN KEY(organisation_id)
        REFERENCES organisations(id) ON DELETE CASCADE
);

-- Give access to the application user.
GRANT SELECT, INSERT, UPDATE, DELETE ON models TO ft_application;
GRANT USAGE, SELECT ON models_id_seq TO ft_application;

-- Give access to the readonly user
GRANT SELECT ON models TO ft_readonly;
GRANT SELECT ON models_id_seq TO ft_readonly;

-- migrate:down
DROP TABLE models;