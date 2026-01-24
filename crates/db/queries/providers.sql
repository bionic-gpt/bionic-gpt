--: Provider(default_model_name?, default_model_display_name?, default_embeddings_model_name?, default_embeddings_model_display_name?, default_embeddings_model_context_size?, default_embeddings_model_description?)

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
    default_embeddings_model_name,
    default_embeddings_model_display_name,
    default_embeddings_model_context_size,
    default_embeddings_model_description,
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
    default_embeddings_model_name,
    default_embeddings_model_display_name,
    default_embeddings_model_context_size,
    default_embeddings_model_description,
    created_at,
    updated_at
FROM
    providers
WHERE
    id = :id;

--! insert(default_model_name?, default_model_display_name?, default_embeddings_model_name?, default_embeddings_model_display_name?, default_embeddings_model_context_size?, default_embeddings_model_description?)
INSERT INTO providers (
    name,
    svg_logo,
    default_model_name,
    default_model_display_name,
    default_model_context_size,
    default_model_description,
    base_url,
    api_key_optional,
    default_embeddings_model_name,
    default_embeddings_model_display_name,
    default_embeddings_model_context_size,
    default_embeddings_model_description
)
VALUES (
    :name,
    :svg_logo,
    :default_model_name,
    :default_model_display_name,
    :default_model_context_size,
    :default_model_description,
    :base_url,
    :api_key_optional,
    :default_embeddings_model_name,
    :default_embeddings_model_display_name,
    :default_embeddings_model_context_size,
    :default_embeddings_model_description
)
RETURNING id;

--! update(default_model_name?, default_model_display_name?, default_embeddings_model_name?, default_embeddings_model_display_name?, default_embeddings_model_context_size?, default_embeddings_model_description?)
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
    api_key_optional = :api_key_optional,
    default_embeddings_model_name = :default_embeddings_model_name,
    default_embeddings_model_display_name = :default_embeddings_model_display_name,
    default_embeddings_model_context_size = :default_embeddings_model_context_size,
    default_embeddings_model_description = :default_embeddings_model_description
WHERE
    id = :id;

--! delete
DELETE FROM
    providers
WHERE
    id = :id;
