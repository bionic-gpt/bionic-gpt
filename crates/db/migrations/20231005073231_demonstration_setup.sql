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
    'http://embeddings-api:80/openai', 
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
    'llama-2-7b', 
    'LLM', 
    'http://llm-api:3000/v1', 
    7, 
    2048
);

-- migrate:down
DELETE FROM models;