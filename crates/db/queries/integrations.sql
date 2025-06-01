--: Integration(configuration?, definition?)

--! integrations : Integration
SELECT
    id,
    name,
    integration_type,
    integration_status,
    configuration,
    definition,
    created_at,
    updated_at
FROM 
    integrations
ORDER BY updated_at;

--! integration : Integration
SELECT
    id,
    name,
    integration_type,
    integration_status,
    configuration,
    definition,
    created_at,
    updated_at
FROM 
    integrations
WHERE
    id = :model_id
ORDER BY updated_at;


--! insert(configuration?, definition?)
INSERT INTO integrations (
    name,
    configuration,
    definition,
    integration_type,
    integration_status
)
VALUES(
    :name,
    :configuration,
    :definition,
    :integration_type,
    :integration_status
)
RETURNING id;

--! update(configuration?, definition?)
UPDATE 
    integrations 
SET 
    name = :name,
    configuration = :configuration,
    definition = :definition,
    integration_type = :integration_type,
    integration_status = :integration_status
WHERE
    id = :id;

--! delete
DELETE FROM
    integrations
WHERE
    id = :id;

--! integrations_for_prompt : Integration
SELECT
    i.id,
    i.name,
    i.integration_type,
    i.integration_status,
    i.configuration,
    i.definition,
    i.created_at,
    i.updated_at
FROM
    integrations i
INNER JOIN
    prompt_integration pi ON i.id = pi.integration_id
WHERE
    pi.prompt_id = :prompt_id
ORDER BY i.updated_at;