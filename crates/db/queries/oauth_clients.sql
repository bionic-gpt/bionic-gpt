--: OauthClient()

--! oauth_clients : OauthClient
SELECT
    id,
    client_id,
    decrypt_text(client_secret) as client_secret,
    provider,
    provider_url,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at
FROM
    oauth_clients
ORDER BY provider, created_at DESC;

--! oauth_client_by_provider : OauthClient
SELECT
    id,
    client_id,
    decrypt_text(client_secret) as client_secret,
    provider,
    provider_url,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at
FROM
    oauth_clients
WHERE
    provider = :provider;

--! oauth_client_by_provider_url : OauthClient
SELECT
    id,
    client_id,
    decrypt_text(client_secret) as client_secret,
    provider,
    provider_url,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at
FROM
    oauth_clients
WHERE
    provider_url = :provider_url;

--! insert_oauth_client
INSERT INTO oauth_clients (
    client_id,
    client_secret,
    provider,
    provider_url
)
VALUES(
    :client_id,
    encrypt_text(:client_secret),
    :provider,
    :provider_url
)
RETURNING id;

--! delete_oauth_client
DELETE FROM
    oauth_clients
WHERE
    id = :id;
