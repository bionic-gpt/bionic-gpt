--: AttachmentObject()
--: AttachmentData()

--! insert
INSERT INTO llm.chats_attachments (
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
    storage.objects o
JOIN
    llm.chats_attachments ca ON o.id = ca.object_id
JOIN
    llm.chats c ON ca.chat_id = c.id
WHERE
    c.conversation_id = :conversation_id;

--! get_content : AttachmentData
SELECT
    o.object_data,
    o.file_name,
    o.mime_type
FROM
    storage.objects o
JOIN
    llm.chats_attachments ca ON o.id = ca.object_id
JOIN
    llm.chats c ON ca.chat_id = c.id
JOIN
    llm.conversations conv ON c.conversation_id = conv.id
WHERE
    o.id = :id
AND
    conv.user_id = current_app_user();

--! get_latest_content : AttachmentData
SELECT
    o.object_data,
    o.file_name,
    o.mime_type
FROM
    storage.objects o
JOIN
    llm.chats_attachments ca ON o.id = ca.object_id
JOIN
    llm.chats ch ON ca.chat_id = ch.id
JOIN
    llm.conversations c ON ch.conversation_id = c.id
WHERE
    c.id = :conversation_id
AND
    c.user_id = current_app_user()
ORDER BY
    ch.id DESC,
    o.id DESC
LIMIT 1;