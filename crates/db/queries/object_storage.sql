--: ObjectStorage(object_data?)

--! get : ObjectStorage
SELECT 
    object_name,
    team_id,
    object_data,
    mime_type,
    file_name,
    file_size,
    file_hash,
    created_by
FROM 
    storage.objects 
WHERE 
    id = :id
LIMIT 1;

--! get_by_team : ObjectStorage
SELECT 
    object_name,
    team_id,
    object_data,
    mime_type,
    file_name,
    file_size,
    file_hash,
    created_by
FROM 
    storage.objects 
WHERE 
    id = :id 
AND team_id = :team_id 
LIMIT 1;

--! insert
INSERT INTO storage.objects (
    object_name,
    team_id,
    object_data,
    mime_type,
    file_name,
    file_size,
    file_hash,
    created_by
) VALUES (
    :object_name,
    :team_id,
    :object_data,
    :mime_type,
    :file_name,
    :file_size,
    :file_hash,
    :created_by
)
RETURNING id;

--! delete
DELETE FROM storage.objects WHERE id = :id;