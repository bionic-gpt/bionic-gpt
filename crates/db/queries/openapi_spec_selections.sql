--: OpenapiSpecSelection()

--! selection : OpenapiSpecSelection
SELECT
    openapi_spec_id
FROM
    openapi_spec_selections
WHERE
    category = :category;

--! set_selection
INSERT INTO openapi_spec_selections (category, openapi_spec_id)
VALUES (:category, :openapi_spec_id)
ON CONFLICT (category)
DO UPDATE SET openapi_spec_id = EXCLUDED.openapi_spec_id, updated_at = NOW();
