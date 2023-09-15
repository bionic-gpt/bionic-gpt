--! datasets : Dataset()
SELECT
    id,
    organisation_id, 
    name,
    (SELECT COUNT(id) FROM documents WHERE dataset_id = d.id) as count,
    created_at,
    updated_at
FROM 
    datasets d
WHERE
    organisation_id IN (SELECT organisation_id FROM organisation_users WHERE user_id = current_app_user())
ORDER BY updated_at;

--! dataset : Dataset()
SELECT
    id,
    organisation_id, 
    name,
    (SELECT COUNT(id) FROM documents WHERE dataset_id = d.id) as count,
    created_at,
    updated_at
FROM 
    datasets d
WHERE
    id = :dataset_id
AND
    organisation_id IN (
        SELECT 
            organisation_id 
        FROM organisation_users WHERE user_id = current_app_user())
ORDER BY updated_at;

--! insert
INSERT INTO 
    datasets (organisation_id, name)
VALUES(:organisation_id, :name)
RETURNING id;