--: PromptIntegration()
--: PromptIntegrationWithConnection(api_connection_id?, oauth2_connection_id?,  definition?, bearer_token?, refresh_token?, expires_at?)

--! prompt_integrations : PromptIntegration
SELECT
    i.id as integration_id,
    pi.prompt_id as prompt_id,
    i.name,
    i.integration_type
FROM 
    integrations.integrations i
LEFT JOIN 
    integrations.prompt_integration pi
ON 
    i.id = pi.integration_id
WHERE
    pi.prompt_id = :prompts_id;

--! delete_prompt_integrations
DELETE FROM integrations.prompt_integration
WHERE
    prompt_id = :prompts_id
AND
    prompt_id IN (
        SELECT id FROM prompting.prompts WHERE model_id IN(
            SELECT id FROM model_registry.models WHERE team_id IN(
                SELECT team_id 
                FROM iam.team_users 
                WHERE user_id = current_app_user()
            )
        )
    );

--! insert_prompt_integration
INSERT INTO integrations.prompt_integration(
    prompt_id,
    integration_id
)
VALUES(
    :prompt_id, :integration_id);

--! insert_prompt_integration_with_connection(api_connection_id?, oauth2_connection_id?)
INSERT INTO integrations.prompt_integration(
    prompt_id,
    integration_id,
    api_connection_id,
    oauth2_connection_id
)
VALUES(
    :prompt_id, 
    :integration_id,
    :api_connection_id,
    :oauth2_connection_id
);

-- This is called by the front end to show the user which integrations.integrations have connections
-- It's also used in the backend to pass the bearer tokens to the open api tool
--! get_prompt_integrations_with_connections : PromptIntegrationWithConnection
SELECT 
    pi.prompt_id,
    pi.integration_id,
    pi.api_connection_id,
    pi.oauth2_connection_id,
    i.name AS integration_name,
    i.definition,
    CASE 
        WHEN akc.api_key IS NOT NULL THEN decrypt_text(akc.api_key)
        WHEN o2c.access_token IS NOT NULL THEN decrypt_text(o2c.access_token)
        ELSE NULL
    END AS bearer_token
    , decrypt_text(o2c.refresh_token) AS refresh_token
    , o2c.expires_at
FROM integrations.prompt_integration pi
JOIN integrations.integrations i ON pi.integration_id = i.id
LEFT JOIN integrations.api_key_connections akc ON pi.api_connection_id = akc.id
LEFT JOIN integrations.oauth2_connections o2c ON pi.oauth2_connection_id = o2c.id
WHERE pi.prompt_id = :prompt_id;

--! delete_specific_prompt_integration
DELETE FROM integrations.prompt_integration
WHERE
    prompt_id = :prompt_id
AND integration_id = :integration_id
AND prompt_id IN (
    SELECT id FROM prompting.prompts WHERE model_id IN(
        SELECT id FROM model_registry.models WHERE team_id IN(
            SELECT team_id
            FROM iam.team_users
            WHERE user_id = current_app_user()
        )
    )
);