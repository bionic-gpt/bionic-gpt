--: WebSearchSelection()

--! selection : WebSearchSelection
SELECT
    openapi_spec_id
FROM
    web_search_specs
WHERE
    id = 1;

--! set_selection
INSERT INTO web_search_specs (id, openapi_spec_id)
VALUES (1, :openapi_spec_id)
ON CONFLICT (id)
DO UPDATE SET openapi_spec_id = EXCLUDED.openapi_spec_id, updated_at = NOW();
