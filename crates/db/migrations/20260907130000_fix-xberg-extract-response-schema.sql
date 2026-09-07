-- migrate:up
UPDATE integrations.openapi_specs
SET spec = jsonb_set(
    spec,
    '{paths,/extract,post,responses,200,content,application/json,schema}',
    '{"type":"object","required":["results"],"properties":{"results":{"type":"array","items":{"type":"object","additionalProperties":true}}}}'::jsonb
)
WHERE slug = 'document-conversion-api';

-- migrate:down
SELECT 1;
