-- migrate:up


-- migrate:down
-- migrate:up

UPDATE integrations.openapi_specs
SET
    title = 'Document Conversion API',
    description = 'Convert business documents into extracted text, tables, and metadata for downstream validation and analysis.',
    updated_at = NOW()
WHERE slug = 'document-conversion-api';

-- migrate:down

UPDATE integrations.openapi_specs
SET
    title = 'Document Conversion API',
    description = 'Extract text and structured content from a supplied document using the internal Xberg document engine.',
    updated_at = NOW()
WHERE slug = 'document-conversion-api';
