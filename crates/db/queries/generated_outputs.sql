--: GeneratedOutput()
--: GeneratedOutputData()

--! list_by_conversation : GeneratedOutput
SELECT
    go.id,
    go.conversation_id,
    go.object_id,
    go.path,
    go.file_name,
    go.mime_type,
    go.file_size,
    go.file_hash,
    go.created_at,
    go.updated_at
FROM
    llm.generated_outputs go
JOIN
    llm.conversations c ON c.id = go.conversation_id
WHERE
    go.conversation_id = :conversation_id
AND
    c.user_id = current_app_user()
ORDER BY
    go.path;

--! get_content : GeneratedOutputData
SELECT
    go.id,
    go.path,
    o.object_data,
    o.file_name,
    o.mime_type,
    o.file_size,
    o.file_hash
FROM
    llm.generated_outputs go
JOIN
    storage.objects o ON o.id = go.object_id
JOIN
    llm.conversations c ON c.id = go.conversation_id
WHERE
    go.id = :id
AND
    c.user_id = current_app_user();

--! find_by_path : GeneratedOutput
SELECT
    go.id,
    go.conversation_id,
    go.object_id,
    go.path,
    go.file_name,
    go.mime_type,
    go.file_size,
    go.file_hash,
    go.created_at,
    go.updated_at
FROM
    llm.generated_outputs go
JOIN
    llm.conversations c ON c.id = go.conversation_id
WHERE
    go.conversation_id = :conversation_id
AND
    go.path = :path
AND
    c.user_id = current_app_user()
LIMIT 1;

--! upsert
INSERT INTO llm.generated_outputs (
    conversation_id,
    object_id,
    path,
    file_name,
    mime_type,
    file_size,
    file_hash
) VALUES (
    :conversation_id,
    :object_id,
    :path,
    :file_name,
    :mime_type,
    :file_size,
    :file_hash
)
ON CONFLICT (conversation_id, path)
DO UPDATE SET
    object_id = EXCLUDED.object_id,
    file_name = EXCLUDED.file_name,
    mime_type = EXCLUDED.mime_type,
    file_size = EXCLUDED.file_size,
    file_hash = EXCLUDED.file_hash,
    updated_at = NOW()
RETURNING id;
