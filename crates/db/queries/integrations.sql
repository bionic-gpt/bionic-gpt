--: Integration(definition?)

--! integrations : Integration
SELECT
    id,
    name,
    integration_type,
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
    definition,
    created_at,
    updated_at
FROM 
    integrations
WHERE
    id = :model_id
ORDER BY updated_at;


--! insert(definition?)
INSERT INTO integrations (
    name,
    definition,
    integration_type
)
VALUES(
    :name,
    :definition,
    :integration_type
)
RETURNING id;

--! update(definition?)
UPDATE 
    integrations 
SET 
    name = :name,
    definition = :definition,
    integration_type = :integration_type
WHERE
    id = :id;

--! delete
DELETE FROM
    integrations
WHERE
    id = :id;