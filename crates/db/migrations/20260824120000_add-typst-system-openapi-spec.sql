-- migrate:up

INSERT INTO integrations.openapi_specs (
    slug,
    title,
    description,
    spec,
    logo_url,
    category,
    is_active,
    is_system
)
VALUES (
    'typst',
    'Typst Compilation API',
    'Compile Typst documents in an isolated workspace.',
    $typst_spec${"openapi":"3.1.0","info":{"title":"Typst Compilation API","version":"1.0.0","description":"Compile Typst documents in an isolated workspace."},"servers":[{"url":"http://cli-gateway:8080"}],"x-cli-build":{"source":{"type":"container","image":"ghcr.io/typst/typst:0.15.1"},"binary":{"from":"/bin/typst","to":"/usr/local/bin/typst"}},"paths":{"/compile":{"post":{"operationId":"compileDocument","summary":"Compile a Typst document","description":"Upload a Typst workspace with main.typ and compile it to output.pdf.","requestBody":{"required":true,"content":{"multipart/form-data":{"schema":{"type":"object","required":["files"],"properties":{"files":{"type":"array","items":{"type":"string","format":"binary"},"description":"Workspace files. Include main.typ and any referenced assets."}}}}}},"x-cli":{"executable":"/usr/local/bin/typst","args":["compile","main.typ","output.pdf"],"output":"output.pdf","timeout-ms":30000},"responses":{"200":{"description":"Compiled PDF document","content":{"application/pdf":{"schema":{"type":"string","format":"binary"}}}},"400":{"description":"Invalid upload"},"422":{"description":"Typst compilation failed"},"504":{"description":"Compilation timed out"}}}}}}$typst_spec$::jsonb,
    NULL,
    'Application',
    TRUE,
    TRUE
)
ON CONFLICT (slug) DO NOTHING;

-- migrate:down

DELETE FROM integrations.openapi_specs
WHERE slug = 'typst'
AND is_system = TRUE;
