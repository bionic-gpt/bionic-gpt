--: ObjectStorage(object_data?)

--! get : ObjectStorage
SELECT * FROM objects WHERE id = :id;

--! insert
INSERT INTO objects (
    object_name,
    team_id,
    object_data,
    mime_type,
    file_name,
    file_size,
    file_hash,
    created_at,
    updated_at
) VALUES (
    :object_name,
    :team_id,
    :object_data,
    :mime_type,
    :file_name,
    :file_size,
    :file_hash,
    :created_at,
    :updated_at
);

--! delete
DELETE FROM objects WHERE id = :id;