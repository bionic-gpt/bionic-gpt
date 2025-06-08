--: Oauth2Connection()

--! insert_oauth2_connection(refresh_token?, expires_at?) : Oauth2Connection
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
--: ApiKeyConnection()

--! insert_api_key_connection : ApiKeyConnection
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