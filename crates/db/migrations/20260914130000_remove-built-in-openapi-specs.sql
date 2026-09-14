-- migrate:up

DELETE FROM integrations.openapi_specs
WHERE slug IN ('typst', 'document-conversion-api')
AND is_system = TRUE;

-- migrate:down

SELECT 1;
