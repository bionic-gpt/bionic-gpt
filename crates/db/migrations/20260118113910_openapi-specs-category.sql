-- migrate:up
CREATE TYPE openapi_spec_category AS ENUM (
    'WebSearch',
    'Application',
    'CodeSandbox'
);

ALTER TABLE openapi_specs
ADD COLUMN category openapi_spec_category NOT NULL DEFAULT 'Application';

INSERT INTO openapi_specs (slug, title, description, spec, logo_url, category) VALUES (
    $$searxng-search$$,
    $$SearXNG Search API (LLM Tool)$$,
    $$OpenAPI specification for the SearXNG /search endpoint, designed for use as a web-search tool by an LLM inside Bionic GPT.$$,
    $${"components":{"securitySchemes":{"ApiKeyAuth":{"description":"API key forwarded by Bionic GPT to authenticate against the SearXNG instance or upstream gateway","in":"header","name":"x-rapidapi-key","type":"apiKey"}}},"info":{"description":"OpenAPI specification for the SearXNG /search endpoint, designed for use as a web-search tool by an LLM inside Bionic GPT.\n\nOperational notes:\n- The SearXNG instance MUST allow JSON output (`json` enabled in `search.formats` in settings.yml).\n- Requests typically fail with HTTP 403 if JSON output is disabled.\n- This spec is intentionally opinionated and simplified for LLM usage, not for full SearXNG feature coverage.","title":"SearXNG Search API (LLM Tool)","version":"1.0.0"},"openapi":"3.1.0","paths":{"/search":{"get":{"description":"Executes a web search query against SearXNG and returns structured JSON results.\n\nThis endpoint should be used by the LLM whenever it needs up-to-date information from the public web.","operationId":"searxng_search","parameters":[{"description":"The search query string. This should be a natural-language query describing what information is being searched for.","in":"query","name":"q","required":true,"schema":{"minLength":1,"type":"string"}},{"description":"Optional category filter (e.g. general, news, science, it). If omitted, SearXNG searches across all enabled categories.","in":"query","name":"categories","required":false,"schema":{"type":"string"}},{"description":"Optional comma-separated list of search engines to use (e.g. google,bing,duckduckgo). If omitted, SearXNG selects default engines.","in":"query","name":"engines","required":false,"schema":{"type":"string"}},{"description":"Optional ISO language code to bias results (e.g. en, fr, de).","in":"query","name":"language","required":false,"schema":{"maxLength":5,"minLength":2,"type":"string"}},{"description":"Page number of the search results (1-based). Use this to retrieve additional results beyond the first page.","in":"query","name":"pageno","required":false,"schema":{"default":1,"minimum":1,"type":"integer"}},{"description":"Response format. MUST be set to `json` for programmatic access. This parameter is fixed for LLM usage.","in":"query","name":"format","required":false,"schema":{"default":"json","enum":["json"],"type":"string"}},{"description":"Safe search level (0 = off, 1 = moderate, 2 = strict).","in":"query","name":"safesearch","required":false,"schema":{"default":0,"enum":[0,1,2],"type":"integer"}}],"responses":{"200":{"content":{"application/json":{"schema":{"description":"SearXNG JSON search response","properties":{"number_of_results":{"description":"Approximate total number of results","type":"integer"},"query":{"description":"The original search query","type":"string"},"results":{"description":"List of individual search results","items":{"properties":{"content":{"description":"Snippet or summary of the result content","type":"string"},"engine":{"description":"Search engine that produced this result","type":"string"},"publishedDate":{"description":"Optional publication date if available","type":"string"},"title":{"description":"Title of the search result","type":"string"},"url":{"description":"Canonical URL of the result","format":"uri","type":"string"}},"required":["title","url"],"type":"object"},"type":"array"}},"required":["results"],"type":"object"}}},"description":"Successful search response"},"403":{"description":"JSON format not allowed or access forbidden"},"500":{"description":"Internal server error"}},"summary":"Search the web using SearXNG","tags":["search"]}}},"security":[{"ApiKeyAuth":[]}],"servers":[{"description":"Base URL of the SearXNG instance","url":"https://{host}","variables":{"host":{"default":"searx.example.org","description":"Hostname (and optional port) of the SearXNG instance"}}}],"tags":[{"description":"Perform web searches using SearXNG","name":"search"}]}$$::jsonb,
    NULL,
    'WebSearch'
) ON CONFLICT (slug) DO NOTHING;

INSERT INTO openapi_specs (slug, title, description, spec, logo_url, category) VALUES (
    $$code-sandbox-runner$$,
    $$CodeSandbox Runner (LLM Tool)$$,
    $$Execute short-lived code snippets inside the sandbox for LLM tool use.$$,
    $${"openapi":"3.1.0","info":{"title":"CodeSandbox Runner (LLM Tool)","version":"1.0.0","description":"Execute short-lived code snippets inside the sandbox for LLM tool use."},"servers":[{"url":"http://sandbox:9000","description":"Internal sandbox runner"}],"paths":{"/run":{"post":{"summary":"Run code in a sandbox","operationId":"run_code","requestBody":{"required":true,"content":{"application/json":{"schema":{"type":"object","properties":{"language":{"type":"string","enum":["python"],"description":"Language runtime to execute"},"code":{"type":"string","description":"Source code to execute"},"stdin":{"type":"string","description":"Optional stdin passed to the program"},"timeout_ms":{"type":"integer","minimum":100,"maximum":300000,"description":"Optional execution timeout in milliseconds"}},"required":["language","code"]}}}},"responses":{"200":{"description":"Execution completed","content":{"application/json":{"schema":{"type":"object","properties":{"stdout":{"type":"string"},"stderr":{"type":"string"},"exit_code":{"type":"integer"},"duration_ms":{"type":"integer"}},"required":["stdout","stderr","exit_code"]}}}},"400":{"description":"Invalid request payload"},"500":{"description":"Execution failed"}}}}}}$$::jsonb,
    NULL,
    'CodeSandbox'
) ON CONFLICT (slug) DO NOTHING;

CREATE TABLE openapi_spec_selections (
    category openapi_spec_category PRIMARY KEY,
    openapi_spec_id INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_openapi_spec_selection FOREIGN KEY(openapi_spec_id)
        REFERENCES openapi_specs(id) ON DELETE CASCADE
);

-- Manage the updated_at column
SELECT updated_at('openapi_spec_selections');

-- Permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON openapi_spec_selections TO application_user;

GRANT SELECT ON openapi_spec_selections TO application_readonly;


-- migrate:down
DROP TABLE openapi_spec_selections;

ALTER TABLE openapi_specs DROP COLUMN category;

DROP TYPE openapi_spec_category;
