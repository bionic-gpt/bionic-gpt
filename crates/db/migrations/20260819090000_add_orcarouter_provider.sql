-- migrate:up
INSERT INTO model_registry.providers (
    name,
    svg_logo,
    default_model_name,
    default_model_display_name,
    default_model_context_size,
    default_model_description,
    base_url
) VALUES (
    'OrcaRouter',
    '<svg width="512" height="512" viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg"><path fill="#0B1220" d="M64 224c0-88 72-160 160-160h64c88 0 160 72 160 160v80c0 88-72 160-160 160h-64C136 464 64 392 64 304v-80zm160-96c-53 0-96 43-96 96v80c0 53 43 96 96 96h64c53 0 96-43 96-96v-80c0-53-43-96-96-96h-64z"/><path fill="#FFFFFF" d="M176 208c0-26 22-48 48-48h64c26 0 48 22 48 48v96c0 26-22 48-48 48h-64c-26 0-48-22-48-48v-96z"/><path fill="#0B1220" d="M176 208h-32v96h32v-96zm160 0h32v96h-32v-96z"/><path fill="#1DA1F2" d="M256 128v256"/></svg>',
    'orcarouter/auto',
    'OrcaRouter Auto',
    131072,
    'OrcaRouter is an OpenAI-compatible model gateway that routes each request to the best model for the job. It also runs gateway-level, zero-trust security for AI agents on the same endpoint.',
    'https://api.orcarouter.ai/v1'
);

-- migrate:down
DELETE FROM model_registry.providers WHERE name = 'OrcaRouter';
