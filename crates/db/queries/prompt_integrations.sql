--: PromptIntegration()
--: PromptIntegrationWithConnection(api_connection_id?, oauth2_connection_id?,  definition?, bearer_token?)

--! prompt_integrations : PromptIntegration
SELECT
    i.id as integration_id,
    pi.prompt_id as prompt_id,
    i.name,
    i.integration_type
FROM 
    integrations i
LEFT JOIN 
    prompt_integration pi
ON 
    i.id = pi.integration_id
WHERE
    pi.prompt_id = :prompts_id;

--! delete_prompt_integrations
DELETE FROM prompt_integration
WHERE
    prompt_id = :prompts_id
AND
    prompt_id IN (
        SELECT id FROM prompts WHERE model_id IN(
            SELECT id FROM models WHERE team_id IN(
                SELECT team_id 
                FROM team_users 
                WHERE user_id = current_app_user()
            )
        )
    );

--! insert_prompt_integration
INSERT INTO prompt_integration(
    prompt_id,
    integration_id
)
VALUES(
    :prompt_id, :integration_id);

--! insert_prompt_integration_with_connection(api_connection_id?, oauth2_connection_id?)
INSERT INTO prompt_integration(
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

-- This is called by the front end to show the user which integrations have connections
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
        WHEN akc.api_key IS NOT NULL THEN akc.api_key
        WHEN o2c.access_token IS NOT NULL THEN o2c.access_token
        ELSE NULL
    END AS bearer_token
FROM prompt_integration pi
JOIN integrations i ON pi.integration_id = i.id
LEFT JOIN api_key_connections akc ON pi.api_connection_id = akc.id
LEFT JOIN oauth2_connections o2c ON pi.oauth2_connection_id = o2c.id
WHERE pi.prompt_id = :prompt_id;

--! delete_specific_prompt_integration
DELETE FROM prompt_integration
WHERE
    prompt_id = :prompt_id
AND integration_id = :integration_id
AND prompt_id IN (
    SELECT id FROM prompts WHERE model_id IN(
        SELECT id FROM models WHERE team_id IN(
            SELECT team_id
            FROM team_users
            WHERE user_id = current_app_user()
        )
    )
);