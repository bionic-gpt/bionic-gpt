-- migrate:up

CREATE EXTENSION vector;

CREATE TABLE datasets (
    id SERIAL PRIMARY KEY, 
    organisation_id INT NOT NULL, 
    name VARCHAR NOT NULL, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_organisation FOREIGN KEY(organisation_id)
        REFERENCES organisations(id) ON DELETE CASCADE
);

CREATE TABLE documents (
    id SERIAL PRIMARY KEY, 
    dataset_id INT NOT NULL, 
    file_name VARCHAR NOT NULL, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_dataset FOREIGN KEY(dataset_id)
        REFERENCES datasets(id) ON DELETE CASCADE
);

CREATE TABLE embeddings (
    id SERIAL PRIMARY KEY, 
    document_id INT NOT NULL, 
    text VARCHAR NOT NULL, 
    embeddings vector(384), 
    processed BOOL NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_document FOREIGN KEY(document_id)
        REFERENCES documents(id) ON DELETE CASCADE
);

-- Give access to the application user.
GRANT SELECT, INSERT, UPDATE, DELETE ON documents TO ft_application;
GRANT USAGE, SELECT ON documents_id_seq TO ft_application;
GRANT SELECT, INSERT, UPDATE, DELETE ON embeddings TO ft_application;
GRANT USAGE, SELECT ON embeddings_id_seq TO ft_application;
GRANT SELECT, INSERT, UPDATE, DELETE ON datasets TO ft_application;
GRANT USAGE, SELECT ON datasets_id_seq TO ft_application;

-- Give access to the readonly user
GRANT SELECT ON embeddings TO ft_readonly;
GRANT SELECT ON embeddings_id_seq TO ft_readonly;
GRANT SELECT ON documents TO ft_readonly;
GRANT SELECT ON documents_id_seq TO ft_readonly;
GRANT SELECT ON datasets TO ft_readonly;
GRANT SELECT ON datasets_id_seq TO ft_readonly;

-- migrate:down
DROP TABLE embeddings;
DROP TABLE documents;
DROP TABLE datasets;
DROP EXTENSION vector;

