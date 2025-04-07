--: Integration()

--! integrations : Integration
SELECT
    id,
    name,
    integration_type,
    base_url,
    created_at,
    updated_at
FROM 
    integrations
WHERE integration_type = :integration_type
ORDER BY updated_at;

--! integration : Integration
SELECT
    id,
    name,
    integration_type,
    base_url,
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
    base_url
)
VALUES(
    :name,
    :integration_type,
    :base_url
)
RETURNING id;

--! update
UPDATE 
    integrations 
SET 
    name = :name,
    integration_type = :integration_type,
    base_url = :base_url
WHERE
    id = :id;

--! delete
DELETE FROM
    integrations
WHERE
    id = :id;