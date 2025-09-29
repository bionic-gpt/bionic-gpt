--: Oauth2Connection()
--: ApiKeyConnection()
--: Oauth2RefreshCandidate(connection_id, integration_id, user_id, team_id, refresh_token?, expires_at?, definition?)

--! insert_oauth2_connection(refresh_token?, expires_at?)
INSERT INTO oauth2_connections (
    integration_id,
    user_id,
    team_id,
    visibility,
    access_token,
    refresh_token,
    expires_at,
    scopes
) VALUES (
    :integration_id,
    current_app_user(),
    :team_id,
    :visibility,
    encrypt_text(:access_token),
    encrypt_text(:refresh_token),
    :expires_at,
    :scopes
) RETURNING id;

--! update_oauth2_connection(refresh_token?, expires_at?)
UPDATE oauth2_connections
SET
    access_token = encrypt_text(:access_token),
    refresh_token = encrypt_text(:refresh_token),
    expires_at = :expires_at
WHERE id = :connection_id;

--! oauth2_connections_needing_refresh : Oauth2RefreshCandidate
SELECT
    oc.id AS connection_id,
    oc.integration_id,
    oc.user_id,
    oc.team_id,
    decrypt_text(oc.refresh_token) AS refresh_token,
    oc.expires_at,
    i.definition
FROM oauth2_connections oc
JOIN integrations i ON oc.integration_id = i.id
WHERE
    oc.refresh_token IS NOT NULL
    AND (oc.expires_at IS NULL OR oc.expires_at <= NOW() + INTERVAL '1 day');

--! insert_api_key_connection
INSERT INTO api_key_connections (
    integration_id,
    user_id,
    team_id,
    visibility,
    api_key
) VALUES (
    :integration_id,
    current_app_user(),
    :team_id,
    :visibility,
    encrypt_text(:api_key)
) RETURNING id;

--! get_api_key_connections_for_integration : ApiKeyConnection
SELECT id, integration_id, user_id, team_id, visibility, external_id, created_at
FROM api_key_connections
WHERE integration_id = :integration_id AND team_id = :team_id;

--! get_oauth2_connections_for_integration : Oauth2Connection
SELECT id, integration_id, user_id, team_id, visibility, external_id, expires_at, scopes, created_at
FROM oauth2_connections
WHERE integration_id = :integration_id AND team_id = :team_id;

--! delete_api_key_connection
DELETE FROM api_key_connections
WHERE id = :connection_id AND team_id = :team_id;

--! delete_oauth2_connection
DELETE FROM oauth2_connections
WHERE id = :connection_id AND team_id = :team_id;

--! get_team_api_key_connections : ApiKeyConnection
SELECT id, integration_id, user_id, team_id, visibility, external_id, created_at
FROM api_key_connections
WHERE team_id = :team_id AND integration_id = :integration_id;

--! get_team_oauth2_connections : Oauth2Connection
SELECT id, integration_id, user_id, team_id, visibility, external_id, expires_at, scopes, created_at
FROM oauth2_connections
WHERE team_id = :team_id AND integration_id = :integration_id;
