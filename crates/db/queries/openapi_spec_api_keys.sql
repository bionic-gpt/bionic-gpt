--: OpenapiSpecApiKey(api_key?)
--: OpenapiSpecApiKeyStatus(has_key)

--! status : OpenapiSpecApiKeyStatus
SELECT
    EXISTS (
        SELECT 1
        FROM openapi_spec_api_keys
        WHERE openapi_spec_id = :openapi_spec_id
    ) AS has_key;

--! api_key : OpenapiSpecApiKey
SELECT
    openapi_spec_id,
    decrypt_text(api_key) AS api_key
FROM
    openapi_spec_api_keys
WHERE
    openapi_spec_id = :openapi_spec_id;

--! upsert
INSERT INTO openapi_spec_api_keys (
    openapi_spec_id,
    api_key
) VALUES (
    :openapi_spec_id,
    encrypt_text(:api_key)
)
ON CONFLICT (openapi_spec_id)
DO UPDATE SET
    api_key = EXCLUDED.api_key,
    updated_at = NOW();

--! delete
DELETE FROM openapi_spec_api_keys
WHERE openapi_spec_id = :openapi_spec_id;
