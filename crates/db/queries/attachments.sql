--: AttachmentObject()
--: AttachmentData()

--! insert
INSERT INTO chats_attachments (
    chat_id,
    object_id
) VALUES (
    :chat_id,
    :object_id
);

--! get_by_conversation : AttachmentObject
SELECT
    o.id,
    o.object_name,
    o.team_id,
    o.mime_type,
    o.file_name,
    o.file_size,
    o.created_by,
    o.created_at
FROM
    objects o
JOIN
    chats_attachments ca ON o.id = ca.object_id
JOIN
    chats c ON ca.chat_id = c.id
WHERE
    c.conversation_id = :conversation_id;

--! get_content : AttachmentData
SELECT
    o.object_data,
    o.file_name,
    o.mime_type
FROM
    objects o
WHERE
    o.id = :id;