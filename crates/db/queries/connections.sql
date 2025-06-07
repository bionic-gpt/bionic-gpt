--: Connection()

--! insert_connection(refresh_token?, expires_at?) : Connection
INSERT INTO connections (
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
    :user_id,
    :team_id,
    :visibility,
    :access_token,
    :refresh_token,
    :expires_at,
    :scopes
) RETURNING id;