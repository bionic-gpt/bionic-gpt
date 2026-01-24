--: Provider(default_model_name?, default_model_display_name?)

--! providers : Provider
SELECT
    id,
    name,
    svg_logo,
    default_model_name,
    default_model_display_name,
    default_model_context_size,
    default_model_description,
    base_url,
    api_key_optional,
    created_at,
    updated_at
FROM
    providers
ORDER BY
    updated_at DESC;

--! provider : Provider
SELECT
    id,
    name,
    svg_logo,
    default_model_name,
    default_model_display_name,
    default_model_context_size,
    default_model_description,
    base_url,
    api_key_optional,
    created_at,
    updated_at
FROM
    providers
WHERE
    id = :id;

--! insert(default_model_name?, default_model_display_name?)
INSERT INTO providers (
    name,
    svg_logo,
    default_model_name,
    default_model_display_name,
    default_model_context_size,
    default_model_description,
    base_url,
    api_key_optional
)
VALUES (
    :name,
    :svg_logo,
    :default_model_name,
    :default_model_display_name,
    :default_model_context_size,
    :default_model_description,
    :base_url,
    :api_key_optional
)
RETURNING id;

--! update(default_model_name?, default_model_display_name?)
UPDATE
    providers
SET
    name = :name,
    svg_logo = :svg_logo,
    default_model_name = :default_model_name,
    default_model_display_name = :default_model_display_name,
    default_model_context_size = :default_model_context_size,
    default_model_description = :default_model_description,
    base_url = :base_url,
    api_key_optional = :api_key_optional
WHERE
    id = :id;

--! delete
DELETE FROM
    providers
WHERE
    id = :id;
