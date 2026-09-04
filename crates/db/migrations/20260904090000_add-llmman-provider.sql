-- migrate:up
-- llmman (https://github.com/llmmanorg/llmman) serves the Ollama API on port
-- 17434, so it reuses the existing Ollama adapter; the logo is a neutral placeholder.
INSERT INTO model_registry.providers (
    name,
    svg_logo,
    default_model_name,
    default_model_display_name,
    default_model_context_size,
    default_model_description,
    base_url,
    provider_type,
    api_key_optional
) VALUES (
    'llmman (Local)',
    '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><title>llmman</title><rect x="3" y="4" width="18" height="16" rx="2"/><path d="m7 9 3 3-3 3"/><path d="M12 15h5"/></svg>',
    'gemma4',
    'Gemma 4',
    32768,
    'Local model served by llmman. Pull it first with `llmman pull gemma4`.',
    'http://host.docker.internal:17434/v1',
    'Ollama',
    true
);

-- migrate:down
DELETE FROM model_registry.providers WHERE name = 'llmman (Local)';
