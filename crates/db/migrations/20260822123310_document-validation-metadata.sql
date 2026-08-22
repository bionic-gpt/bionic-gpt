-- migrate:up


-- migrate:down
-- migrate:up

UPDATE integrations.openapi_specs
SET
    title = 'Document Extraction',
    description = 'Extract text, tables, and metadata from supplied business documents for downstream validation and analysis.',
    updated_at = NOW()
WHERE slug = 'xberg-doc-engine';

-- migrate:down

UPDATE integrations.openapi_specs
SET
    title = 'Xberg Document Engine',
    description = 'Extract text and structured content from a supplied document using the internal Xberg document engine.',
    updated_at = NOW()
WHERE slug = 'xberg-doc-engine';
