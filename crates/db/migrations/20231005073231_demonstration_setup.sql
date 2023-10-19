-- migrate:up
INSERT INTO users (
    email,
    hashed_password
)
VALUES(
    'ian@bionic-gpt.com',
    '$argon2id$v=19$m=4096,t=3,p=1$f6q8zCzaKWTUQGRF/Ydy9Q$0zj6jas3IN7wdh9BEJY9vJ50TegKzz+qVzIAbzPVUv4'
);

INSERT INTO organisations (
    created_by_user_id
)
VALUES(
    (SELECT id FROM users LIMIT 1)
);

INSERT INTO organisation_users (
    user_id,
    organisation_id,
    roles
)
VALUES(
    (SELECT id FROM users LIMIT 1),
    (SELECT id FROM organisations LIMIT 1),
    ARRAY['Administrator', 'Collaborator', 'SystemAdministrator']::role[]
);

INSERT INTO models (
    name,
    model_type,
    base_url,
    billion_parameters,
    context_size
)
VALUES(
    'llama-2-7b', 
    'LLM', 
    'http://local-ai:3000/v1', 
    7, 
    4096
);

INSERT INTO models (
    name,
    model_type,
    base_url,
    billion_parameters,
    context_size
)
VALUES(
    'text-embedding-ada-002', 
    'Embeddings', 
    'http://local-ai:3000', 
    7, 
    2048
);

INSERT INTO prompts (
    model_id, 
    organisation_id,
    visibility,
    name,
    dataset_connection,
    max_history_items,
    max_chunks,
    max_tokens
)
VALUES(
    (SELECT id FROM models WHERE model_type = 'LLM' LIMIT 1), 
    (SELECT id FROM organisations LIMIT 1),
    'Company',
    'Llama 2 7b (All Datasets)', 
    'All', 
    0,
    3,
    2048
);

-- migrate:down
DELETE FROM prompts;
DELETE FROM models;
DELETE FROM organisations;
DELETE FROM organisation_users;
DELETE FROM users;

