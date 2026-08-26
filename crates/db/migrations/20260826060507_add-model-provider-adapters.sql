-- migrate:up
CREATE TYPE model_provider AS ENUM (
    'OpenAI',
    'Groq',
    'OpenRouter',
    'Ollama',
    'OpenAICompatible'
);

ALTER TABLE model_registry.providers
ADD COLUMN provider_type model_provider NOT NULL DEFAULT 'OpenAICompatible';

UPDATE model_registry.providers
SET provider_type = CASE name
    WHEN 'OpenAI' THEN 'OpenAI'::model_provider
    WHEN 'Groq' THEN 'Groq'::model_provider
    WHEN 'OpenRouter' THEN 'OpenRouter'::model_provider
    WHEN 'Ollama (Local)' THEN 'Ollama'::model_provider
    ELSE 'OpenAICompatible'::model_provider
END;

ALTER TABLE model_registry.models
ADD COLUMN provider_type model_provider NOT NULL DEFAULT 'OpenAICompatible';

UPDATE model_registry.models AS model
SET provider_type = provider.provider_type
FROM model_registry.providers AS provider
WHERE RTRIM(model.base_url, '/') = RTRIM(provider.base_url, '/');


-- migrate:down
ALTER TABLE model_registry.models DROP COLUMN provider_type;
ALTER TABLE model_registry.providers DROP COLUMN provider_type;
DROP TYPE model_provider;
