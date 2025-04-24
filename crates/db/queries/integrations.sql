--: Integration(configuration?)

--! integrations : Integration
SELECT
    id,
    name,
    integration_type,
    integration_status,
    configuration,
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
    created_at,
    updated_at
FROM 
    integrations
WHERE
    id = :model_id
ORDER BY updated_at;


--! insert
INSERT INTO integrations (
    name,
    integration_type,
    integration_status
)
VALUES(
    :name,
    :integration_type,
    :integration_status
)
RETURNING id;

--! update
UPDATE 
    integrations 
SET 
    name = :name,
    integration_type = :integration_type,
    integration_status = :integration_status
WHERE
    id = :id;

--! delete
DELETE FROM
    integrations
WHERE
    id = :id;