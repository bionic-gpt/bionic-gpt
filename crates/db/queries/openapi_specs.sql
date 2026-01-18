--: OpenapiSpec(description?, logo_url?)

--! list : OpenapiSpec
SELECT
    id,
    slug,
    title,
    description,
    spec,
    logo_url,
    category,
    is_active,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at,
    trim(both '"' from to_json(updated_at)::text) as updated_at
FROM
    openapi_specs
ORDER BY
    title;

--! active : OpenapiSpec
SELECT
    id,
    slug,
    title,
    description,
    spec,
    logo_url,
    category,
    is_active,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at,
    trim(both '"' from to_json(updated_at)::text) as updated_at
FROM
    openapi_specs
WHERE
    is_active = TRUE
AND
    category != 'WebSearch'
ORDER BY
    title;

--! by_id : OpenapiSpec
SELECT
    id,
    slug,
    title,
    description,
    spec,
    logo_url,
    category,
    is_active,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at,
    trim(both '"' from to_json(updated_at)::text) as updated_at
FROM
    openapi_specs
WHERE
    id = :id;

--! insert(description?, logo_url?)
INSERT INTO openapi_specs (
    slug,
    title,
    description,
    spec,
    logo_url,
    category,
    is_active
)
VALUES (
    :slug,
    :title,
    :description,
    :spec,
    :logo_url,
    :category,
    :is_active
)
RETURNING id;

--! update(description?, logo_url?)
UPDATE openapi_specs
SET
    slug = :slug,
    title = :title,
    description = :description,
    spec = :spec,
    logo_url = :logo_url,
    category = :category,
    is_active = :is_active,
    updated_at = NOW()
WHERE
    id = :id;

--! web_search : OpenapiSpec
SELECT
    id,
    slug,
    title,
    description,
    spec,
    logo_url,
    category,
    is_active,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at,
    trim(both '"' from to_json(updated_at)::text) as updated_at
FROM
    openapi_specs
WHERE
    category = 'WebSearch'
ORDER BY
    title;

--! delete
DELETE FROM openapi_specs
WHERE
    id = :id;
