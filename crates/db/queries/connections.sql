--: Oauth2Connection()
--: ApiKeyConnection()
--: Oauth2RefreshCandidate(connection_id, integration_id, user_id, team_id, refresh_token?, expires_at?, definition?)
--: McpConnectionContext(connection_type, connection_id, integration_id, user_id, user_openid_sub?, definition?)
--: McpApiKeySecret(connection_id, integration_id, user_id, user_openid_sub?, api_key?, definition?)
--: McpOauth2Secret(connection_id, integration_id, user_id, user_openid_sub?, access_token?, refresh_token?, expires_at?, definition?)

--! insert_oauth2_connection(refresh_token?, expires_at?)
INSERT INTO integrations.oauth2_connections (
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
UPDATE integrations.oauth2_connections
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
FROM integrations.oauth2_connections oc
JOIN integrations.integrations i ON oc.integration_id = i.id
WHERE
    oc.refresh_token IS NOT NULL
    AND (oc.expires_at IS NULL OR oc.expires_at <= NOW() + INTERVAL '1 day');

--! insert_api_key_connection
INSERT INTO integrations.api_key_connections (
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
SELECT
    id,
    integration_id,
    user_id,
    team_id,
    visibility,
    external_id,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at
FROM integrations.api_key_connections
WHERE integration_id = :integration_id AND team_id = :team_id;

--! get_oauth2_connections_for_integration : Oauth2Connection
SELECT
    id,
    integration_id,
    user_id,
    team_id,
    visibility,
    external_id,
    expires_at,
    scopes,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at
FROM integrations.oauth2_connections
WHERE integration_id = :integration_id AND team_id = :team_id;

--! delete_api_key_connection
DELETE FROM integrations.api_key_connections
WHERE id = :connection_id AND team_id = :team_id;

--! delete_oauth2_connection
DELETE FROM integrations.oauth2_connections
WHERE id = :connection_id AND team_id = :team_id;

--! get_team_api_key_connections : ApiKeyConnection
SELECT
    id,
    integration_id,
    user_id,
    team_id,
    visibility,
    external_id,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at
FROM integrations.api_key_connections
WHERE team_id = :team_id AND integration_id = :integration_id;

--! get_team_oauth2_connections : Oauth2Connection
SELECT
    id,
    integration_id,
    user_id,
    team_id,
    visibility,
    external_id,
    expires_at,
    scopes,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at
FROM integrations.oauth2_connections
WHERE team_id = :team_id AND integration_id = :integration_id;

--! mcp_connection_context : McpConnectionContext
SELECT
    ctx.connection_type,
    ctx.connection_id,
    ctx.integration_id,
    ctx.user_id,
    ctx.user_openid_sub,
    ctx.definition
FROM (
    SELECT
        'api_key'::text AS connection_type,
        c.id AS connection_id,
        c.integration_id,
        c.user_id,
        u.openid_sub AS user_openid_sub,
        i.definition
    FROM integrations.integrations i
    JOIN integrations.api_key_connections c ON c.integration_id = i.id
    JOIN auth.users u ON u.id = c.user_id
    WHERE LOWER(COALESCE(i.definition->'info'->>'x-bionic-slug', i.definition->'info'->>'bionic-slug')) = LOWER(:slug)
      AND c.external_id = :external_id

    UNION ALL

    SELECT
        'oauth2'::text AS connection_type,
        c.id AS connection_id,
        c.integration_id,
        c.user_id,
        u.openid_sub AS user_openid_sub,
        i.definition
    FROM integrations.integrations i
    JOIN integrations.oauth2_connections c ON c.integration_id = i.id
    JOIN auth.users u ON u.id = c.user_id
    WHERE LOWER(COALESCE(i.definition->'info'->>'x-bionic-slug', i.definition->'info'->>'bionic-slug')) = LOWER(:slug)
      AND c.external_id = :external_id
) AS ctx
LIMIT 1;

--! mcp_api_key_connection_secret : McpApiKeySecret
SELECT
    c.id AS connection_id,
    c.integration_id,
    c.user_id,
    u.openid_sub AS user_openid_sub,
    decrypt_text(c.api_key) AS api_key,
    i.definition
FROM integrations.integrations i
JOIN integrations.api_key_connections c ON c.integration_id = i.id
JOIN auth.users u ON u.id = c.user_id
WHERE LOWER(COALESCE(i.definition->'info'->>'x-bionic-slug', i.definition->'info'->>'bionic-slug')) = LOWER(:slug)
  AND c.external_id = :external_id
LIMIT 1;

--! mcp_oauth2_connection_secret : McpOauth2Secret
SELECT
    c.id AS connection_id,
    c.integration_id,
    c.user_id,
    u.openid_sub AS user_openid_sub,
    decrypt_text(c.access_token) AS access_token,
    decrypt_text(c.refresh_token) AS refresh_token,
    c.expires_at,
    i.definition
FROM integrations.integrations i
JOIN integrations.oauth2_connections c ON c.integration_id = i.id
JOIN auth.users u ON u.id = c.user_id
WHERE LOWER(COALESCE(i.definition->'info'->>'x-bionic-slug', i.definition->'info'->>'bionic-slug')) = LOWER(:slug)
  AND c.external_id = :external_id
LIMIT 1;
