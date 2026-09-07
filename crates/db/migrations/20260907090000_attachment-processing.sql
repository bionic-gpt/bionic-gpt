-- migrate:up
ALTER TABLE llm.chats_attachments
    ADD COLUMN IF NOT EXISTS content_object_id INT REFERENCES storage.objects(id) ON DELETE SET NULL;

UPDATE ops.runtime_settings
SET value = regexp_replace(
    regexp_replace(
        regexp_replace(
            value,
            E'(^|\\n)- /home/user/work[^\\n]*',
            E'\\1',
            'g'
        ),
        E'(^|\\n)- /home/user/attachments[^\\n]*',
        E'\\1- /home/user/attachments - uploaded chat files; each conversation includes a compact Current attachments manifest with original and extracted Markdown paths',
        'g'
    ),
    E'(^|\\n)- /home/user/output[^\\n]*',
    E'\\1- /home/user/work - persistent workspace for extracted intermediate files; contents survive tool calls but do not appear in chat' || chr(10) || '- /home/user/output - persistent workspace for generated files and state; contents survive tool calls and generated artifacts appear in chat',
    'g'
)
WHERE key = 'default_system_prompt';

-- migrate:down
ALTER TABLE llm.chats_attachments
    DROP COLUMN IF EXISTS content_object_id;
