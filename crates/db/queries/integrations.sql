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