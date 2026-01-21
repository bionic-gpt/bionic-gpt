-- migrate:up

INSERT INTO openapi_specs (slug, title, description, spec, logo_url, category) VALUES (
    $$serper-search$$,
    $$Serper Search API$$,
    $$OpenAPI specification for the Serper Google Search endpoint. Authentication is performed using an API key passed via the X-API-KEY header.$$,
    $${"openapi":"3.1.0","info":{"title":"Serper Search API","version":"1.0.0","description":"OpenAPI specification for the Serper Google Search endpoint. Authentication is performed using an API key passed via the X-API-KEY header."},"servers":[{"url":"https://google.serper.dev"}],"security":[{"ApiKeyAuth":[]}],"paths":{"/search":{"post":{"summary":"Google web search","operationId":"serperSearch","description":"Performs a Google-style web search and returns organic results, snippets, and metadata depending on the query.","security":[{"ApiKeyAuth":[]}],"requestBody":{"required":true,"content":{"application/json":{"schema":{"$ref":"#/components/schemas/SearchRequest"},"examples":{"basic":{"summary":"Basic search","value":{"q":"openai api pricing"}},"withOptions":{"summary":"Search with locale and result count","value":{"q":"kubernetes ingress controller","hl":"en","gl":"us","num":10}}}}}},"responses":{"200":{"description":"Search results","content":{"application/json":{"schema":{"$ref":"#/components/schemas/SearchResponse"}}}},"401":{"description":"Unauthorized - missing or invalid API key"},"429":{"description":"Too many requests - rate or credit limit exceeded"},"500":{"description":"Server error"}}}}},"components":{"securitySchemes":{"ApiKeyAuth":{"type":"apiKey","in":"header","name":"X-API-KEY","description":"Serper API key"}},"schemas":{"SearchRequest":{"type":"object","required":["q"],"properties":{"q":{"type":"string","description":"Search query"},"hl":{"type":"string","description":"Host language (e.g. \"en\")"},"gl":{"type":"string","description":"Geo location (e.g. \"us\")"},"num":{"type":"integer","description":"Number of results to return","minimum":1,"maximum":100}}},"SearchResponse":{"type":"object","description":"Response schema is intentionally loose because Serper returns variable fields depending on SERP features.","additionalProperties":true}}}}$$::jsonb,
    NULL,
    'WebSearch'
) ON CONFLICT (slug) DO NOTHING;

-- migrate:down

DELETE FROM openapi_specs WHERE slug = 'serper-search';
