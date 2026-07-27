--: Oauth2Connection()
--: ApiKeyConnection()
--: Oauth2RefreshCandidate(connection_id, integration_id, user_id, team_id, refresh_token?, expires_at?, definition?)
--: McpConnectionContext(connection_type, connection_id, integration_id, user_id, user_openid_sub?, definition?)
--: McpApiKeySecret(connection_id, integration_id, user_id, user_openid_sub?, api_key?, definition?)
--: McpOauth2Secret(connection_id, integration_id, user_id, user_openid_sub?, access_token?, refresh_token?, expires_at?, definition?)
--: ConnectedIntegration(api_connection_id?, oauth2_connection_id?, definition?, bearer_token?, refresh_token?, expires_at?)

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
)
SELECT
    connection.integration_id,
    connection.user_id,
    connection.team_id,
    connection.visibility,
    connection.access_token,
    connection.refresh_token,
    connection.expires_at,
    connection.scopes
FROM (
    VALUES (
        :integration_id::INT,
        current_app_user(),
        :team_id::INT,
        :visibility::visibility,
        encrypt_text(:access_token::TEXT),
        encrypt_text(:refresh_token::TEXT),
        :expires_at::TIMESTAMPTZ,
        :scopes::JSONB
    )
) AS connection (
    integration_id,
    user_id,
    team_id,
    visibility,
    access_token,
    refresh_token,
    expires_at,
    scopes
)
WHERE EXISTS (
    SELECT 1
    FROM integrations.integrations i
    WHERE i.id = connection.integration_id
    AND i.team_id = connection.team_id
)
RETURNING id;

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
)
SELECT
    connection.integration_id,
    connection.user_id,
    connection.team_id,
    connection.visibility,
    connection.api_key
FROM (
    VALUES (
        :integration_id::INT,
        current_app_user(),
        :team_id::INT,
        :visibility::visibility,
        encrypt_text(:api_key::TEXT)
    )
) AS connection (
    integration_id,
    user_id,
    team_id,
    visibility,
    api_key
)
WHERE EXISTS (
    SELECT 1
    FROM integrations.integrations i
    WHERE i.id = connection.integration_id
    AND i.team_id = connection.team_id
)
RETURNING id;

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

--! connected_integrations : ConnectedIntegration
WITH ranked_connections AS (
    SELECT
        i.id AS integration_id,
        i.name AS integration_name,
        i.definition,
        akc.id AS api_connection_id,
        NULL::INT AS oauth2_connection_id,
        decrypt_text(akc.api_key) AS bearer_token,
        NULL::TEXT AS refresh_token,
        NULL::TIMESTAMPTZ AS expires_at,
        akc.created_at,
        akc.id AS connection_sort_id
    FROM integrations.integrations i
    JOIN integrations.api_key_connections akc ON akc.integration_id = i.id
    WHERE i.team_id = :team_id
      AND akc.team_id = :team_id

    UNION ALL

    SELECT
        i.id AS integration_id,
        i.name AS integration_name,
        i.definition,
        NULL::INT AS api_connection_id,
        o2c.id AS oauth2_connection_id,
        decrypt_text(o2c.access_token) AS bearer_token,
        decrypt_text(o2c.refresh_token) AS refresh_token,
        o2c.expires_at,
        o2c.created_at,
        o2c.id AS connection_sort_id
    FROM integrations.integrations i
    JOIN integrations.oauth2_connections o2c ON o2c.integration_id = i.id
    WHERE i.team_id = :team_id
      AND o2c.team_id = :team_id
),
authless_integrations AS (
    SELECT
        i.id AS integration_id,
        i.name AS integration_name,
        i.definition,
        NULL::INT AS api_connection_id,
        NULL::INT AS oauth2_connection_id,
        NULL::TEXT AS bearer_token,
        NULL::TEXT AS refresh_token,
        NULL::TIMESTAMPTZ AS expires_at,
        i.created_at,
        0 AS connection_sort_id
    FROM integrations.integrations i
    WHERE i.team_id = :team_id
      AND i.definition IS NOT NULL
      AND NOT EXISTS (
          SELECT 1
          FROM jsonb_each(COALESCE(i.definition->'components'->'securitySchemes', '{}'::jsonb)) AS security_scheme(name, scheme)
          WHERE scheme->>'type' IN ('apiKey', 'oauth2')
      )
)
SELECT
    integration_id,
    integration_name,
    definition,
    api_connection_id,
    oauth2_connection_id,
    bearer_token,
    refresh_token,
    expires_at
FROM (
    SELECT
        integration_id,
        integration_name,
        definition,
        api_connection_id,
        oauth2_connection_id,
        bearer_token,
        refresh_token,
        expires_at
    FROM (
        SELECT
            *,
            ROW_NUMBER() OVER (
                PARTITION BY integration_id
                ORDER BY created_at DESC, connection_sort_id DESC
            ) AS rank
        FROM ranked_connections
    ) ranked
    WHERE rank = 1
    UNION ALL
    SELECT
        integration_id,
        integration_name,
        definition,
        api_connection_id,
        oauth2_connection_id,
        bearer_token,
        refresh_token,
        expires_at
    FROM authless_integrations
) connected
ORDER BY integration_name, integration_id;

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
    JOIN iam.users u ON u.id = c.user_id
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
    JOIN iam.users u ON u.id = c.user_id
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
JOIN iam.users u ON u.id = c.user_id
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
JOIN iam.users u ON u.id = c.user_id
WHERE LOWER(COALESCE(i.definition->'info'->>'x-bionic-slug', i.definition->'info'->>'bionic-slug')) = LOWER(:slug)
  AND c.external_id = :external_id
LIMIT 1;
