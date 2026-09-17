-- migrate:up
-- Ollama is the fallback embedding provider for LLM providers that do not
-- expose an embeddings endpoint, such as Groq. Preserve any administrator
-- overrides while repairing incomplete provider seed data.
UPDATE model_registry.providers
SET default_embeddings_model_name = COALESCE(
        default_embeddings_model_name,
        'nomic-embed-text'
    ),
    default_embeddings_model_display_name = COALESCE(
        default_embeddings_model_display_name,
        'Nomic Embed Text'
    ),
    default_embeddings_model_context_size = COALESCE(
        default_embeddings_model_context_size,
        8192
    ),
    default_embeddings_model_description = COALESCE(
        default_embeddings_model_description,
        'Local embeddings model optimized for Ollama.'
    )
WHERE provider_type = 'Ollama';

-- migrate:down
-- The update preserves existing and administrator-provided values, so the
-- original state cannot be reconstructed safely.
SELECT 1;
