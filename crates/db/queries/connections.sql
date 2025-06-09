--: Oauth2Connection()
--: ApiKeyConnection()

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
    :access_token,
    :refresh_token,
    :expires_at,
    :scopes
) RETURNING id;

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
    :api_key
) RETURNING id;

--! get_api_key_connections_for_integration : ApiKeyConnection
SELECT id, integration_id, user_id, team_id, visibility, created_at
FROM api_key_connections
WHERE integration_id = :integration_id AND team_id = :team_id;

--! get_oauth2_connections_for_integration : Oauth2Connection
SELECT id, integration_id, user_id, team_id, visibility, expires_at, scopes, created_at
FROM oauth2_connections
WHERE integration_id = :integration_id AND team_id = :team_id;

--! delete_api_key_connection
DELETE FROM api_key_connections
WHERE id = :connection_id AND team_id = :team_id;

--! delete_oauth2_connection
DELETE FROM oauth2_connections
WHERE id = :connection_id AND team_id = :team_id;