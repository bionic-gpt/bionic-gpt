-- migrate:up
-- The LiteLLM provider imports every model alias the proxy serves, with the
-- context window and capabilities its /model/info endpoint reports, instead
-- of creating a single default model.
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
)
SELECT
    'LiteLLM',
    '<svg fill="currentColor" fill-rule="evenodd" height="1em" style="flex:none;line-height:1" viewBox="0 0 24 24" width="1em" xmlns="http://www.w3.org/2000/svg"><title>LiteLLM</title><path d="M3 4h3v13h6v3H3V4zm10 0h3v13h5v3h-8V4z"></path></svg>',
    NULL,
    NULL,
    0,
    'Import the models a LiteLLM proxy serves, with their context windows and capabilities.',
    'http://localhost:4000/v1',
    'LiteLLM',
    TRUE
WHERE NOT EXISTS (
    SELECT 1 FROM model_registry.providers WHERE provider_type = 'LiteLLM'
);

-- migrate:down
DELETE FROM model_registry.providers WHERE provider_type = 'LiteLLM';
