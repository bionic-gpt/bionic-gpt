-- migrate:up
INSERT INTO users (
    email,
    hashed_password
)
VALUES(
    'ian@bionic-gpt.com',
    '$argon2id$v=19$m=4096,t=3,p=1$UNbyHgC0MTBehD5cLXihcQ$FlVWS8729FJcYPlIJzhP47/k1K3RAQ5DAWDsaWGwod4'
);

INSERT INTO organisations (
    created_by_user_id
)
VALUES(
    (SELECT id FROM users LIMIT 1)
);

INSERT INTO models (
    name,
    model_type,
    base_url,
    billion_parameters,
    context_size
)
VALUES(
    'ggml-gpt4all-j', 
    'LLM', 
    'http://llm-api:8080', 
    70, 
    2048
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
    'http://llm-api:8080', 
    70, 
    2048
);

INSERT INTO prompts (
    model_id, 
    organisation_id,
    visibility,
    name,
    dataset_connection,
    template,
    min_history_items,
    max_history_items,
    max_tokens
)
VALUES(
    (SELECT id FROM models WHERE model_type = 'LLM' LIMIT 1), 
    (SELECT id FROM organisations LIMIT 1),
    'Company',
    'GPT4All (All Datasets)', 
    'All', 
    'Context information is below. \n--------------------\n{context_str}\n--------------------',
    3,
    10,
    1024
);

-- migrate:down
DELETE FROM prompts;
DELETE FROM models;
DELETE FROM organisations;
DELETE FROM users;

