-- migrate:up

INSERT INTO models (
    name,
    model_type,
    base_url,
    billion_parameters,
    context_size
)
VALUES(
    'bge-small-en-v1.5', 
    'Embeddings', 
    'http://embeddings-api:80/embeddings', 
    0, 
    0
);

INSERT INTO models (
    name,
    model_type,
    base_url,
    billion_parameters,
    context_size
)
VALUES(
    'llama3', 
    'LLM', 
    'http://llm-api:11434/v1', 
    7, 
    2048
);

-- migrate:down
DELETE FROM models;