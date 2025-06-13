--: OauthClient()

--! oauth_clients : OauthClient
SELECT
    id,
    client_id,
    client_secret,
    provider,
    created_at
FROM 
    oauth_clients
ORDER BY provider, created_at DESC;

--! oauth_client_by_provider : OauthClient
SELECT
    id,
    client_id,
    client_secret,
    provider,
    created_at
FROM 
    oauth_clients
WHERE
    provider = :provider;

--! insert_oauth_client
INSERT INTO oauth_clients (
    client_id,
    client_secret,
    provider
)
VALUES(
    :client_id,
    :client_secret,
    :provider
)
RETURNING id;

--! delete_oauth_client
DELETE FROM
    oauth_clients
WHERE
    id = :id;