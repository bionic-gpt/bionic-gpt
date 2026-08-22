-- migrate:up

INSERT INTO integrations.openapi_specs (
    slug,
    title,
    description,
    spec,
    logo_url,
    category,
    is_active
)
VALUES (
    'xberg-doc-engine',
    'Xberg Document Engine',
    'Extract text and structured content from a supplied document using the internal Xberg document engine.',
    $spec$
    {
      "openapi": "3.0.3",
      "info": {
        "title": "Xberg Document Engine",
        "version": "1.0.14",
        "description": "Internal document extraction service for business documents. Provide the file as base64 encoded bytes. The service returns extracted content and metadata.",
        "x-bionic-slug": "xberg-doc-engine"
      },
      "servers": [{"url": "http://doc-engine:8000"}],
      "paths": {
        "/extract": {
          "post": {
            "operationId": "extractDocument",
            "summary": "Extract a business document",
            "description": "Extract text, tables, and metadata from a supplied document. Read the selected file from the conversation VFS, base64 encode its bytes, and pass them as `files`. Use this when document extraction is needed; do not invent content that is not present in the response.",
            "requestBody": {
              "required": true,
              "content": {
                "multipart/form-data": {
                  "schema": {
                    "type": "object",
                    "required": ["files"],
                    "properties": {
                      "files": {"type": "string", "format": "byte", "description": "Base64-encoded bytes of the document to extract."},
                      "chunking_strategy": {"type": "string", "default": "by_title"},
                      "combine_under_n_chars": {"type": "integer", "default": 500},
                      "new_after_n_chars": {"type": "integer", "default": 1500},
                      "multipage_sections": {"type": "boolean", "default": true}
                    }
                  }
                }
              }
            },
            "responses": {
              "200": {
                "description": "Extracted document content",
                "content": {"application/json": {"schema": {"type": "array", "items": {"type": "object", "additionalProperties": true}}}}
              },
              "400": {"description": "Invalid document or request"},
              "415": {"description": "Unsupported document type"},
              "500": {"description": "Document extraction failed"}
            }
          }
        }
      }
    }
    $spec$::jsonb,
    NULL,
    'Application',
    TRUE
)
ON CONFLICT (slug) DO UPDATE SET
    title = EXCLUDED.title,
    description = EXCLUDED.description,
    spec = EXCLUDED.spec,
    category = EXCLUDED.category,
    is_active = EXCLUDED.is_active,
    updated_at = NOW();

-- migrate:down
DELETE FROM integrations.openapi_specs WHERE slug = 'xberg-doc-engine';
