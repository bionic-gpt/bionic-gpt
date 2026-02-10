-- migrate:up
ALTER TABLE providers
    ADD COLUMN api_key_optional BOOLEAN NOT NULL DEFAULT false;

ALTER TABLE providers
    ADD COLUMN default_embeddings_model_name VARCHAR,
    ADD COLUMN default_embeddings_model_display_name VARCHAR,
    ADD COLUMN default_embeddings_model_context_size INT,
    ADD COLUMN default_embeddings_model_description TEXT;

UPDATE providers
SET api_key_optional = true
WHERE name = 'Ollama';

UPDATE providers
SET default_embeddings_model_name = 'nomic-embed-text',
    default_embeddings_model_display_name = 'Nomic Embed Text',
    default_embeddings_model_context_size = 8192,
    default_embeddings_model_description = 'Local embeddings model optimized for Ollama.'
WHERE name = 'Ollama';

UPDATE providers
SET default_embeddings_model_name = 'nomic-embed-text',
    default_embeddings_model_display_name = 'Nomic Embed Text',
    default_embeddings_model_context_size = 8192,
    default_embeddings_model_description = 'Nomic embeddings model via OpenRouter.'
WHERE name = 'OpenRouter';

UPDATE providers
SET default_embeddings_model_name = 'text-embedding-3-small',
    default_embeddings_model_display_name = 'Text Embedding 3 Small',
    default_embeddings_model_context_size = 8191,
    default_embeddings_model_description = 'Efficient embedding model for general-purpose search and retrieval.'
WHERE name = 'OpenAI';

UPDATE providers
SET default_embeddings_model_name = 'Qwen/Qwen3-Embedding-8B',
    default_embeddings_model_display_name = 'Qwen3 Embedding 8B',
    default_embeddings_model_context_size = 32768,
    default_embeddings_model_description = 'Qwen3 embedding model via Doubleword.'
WHERE name = 'Doubleword';

-- migrate:down
ALTER TABLE providers
    DROP COLUMN api_key_optional;

ALTER TABLE providers
    DROP COLUMN default_embeddings_model_name,
    DROP COLUMN default_embeddings_model_display_name,
    DROP COLUMN default_embeddings_model_context_size,
    DROP COLUMN default_embeddings_model_description;
