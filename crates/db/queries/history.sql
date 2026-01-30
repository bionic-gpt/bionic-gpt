--: History(prompt_id?)

--! search_history : History
WITH search_results AS (
    SELECT
        c.conversation_id,
        c.prompt_id,
        c.created_at,
        decrypt_text(c.content) as content,
        p.prompt_type
    FROM
        llm.chats c
    JOIN
        llm.conversations conv ON c.conversation_id = conv.id
    JOIN
        prompting.prompts p ON c.prompt_id = p.id
    WHERE
        conv.user_id = :user_id
    AND LOWER(decrypt_text(c.content)) LIKE LOWER('%' || :search_term || '%')
)
SELECT
    sr.conversation_id as id,
    sr.prompt_id as prompt_id,
    LEFT(COALESCE(sr.content), 255) as summary,
    trim(both '"' from to_json(sr.created_at)::text) as created_at_iso,
    sr.created_at,
    sr.prompt_type
FROM
    search_results sr
ORDER BY
    sr.created_at DESC
LIMIT :limit;


--! history : History
WITH summary AS (
    SELECT * FROM llm.chats
    WHERE id IN (SELECT MIN(id) FROM llm.chats GROUP BY conversation_id)
)
SELECT
    c.id,
    summary.prompt_id,
    CASE
        WHEN LENGTH(decrypt_text(summary.content)) > 150 THEN
            LEFT(decrypt_text(summary.content), 150) || '...'
        ELSE
            decrypt_text(summary.content)
    END AS summary,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(c.created_at)::text) as created_at_iso,
    c.created_at,
    p.prompt_type
FROM
    llm.conversations c
JOIN
    summary
ON
    c.id = summary.conversation_id
JOIN
    prompting.prompts p ON summary.prompt_id = p.id
WHERE
    c.user_id = current_app_user()
AND
    -- Make sure the user has access to this conversation
    c.team_id IN (
        SELECT team_id
        FROM tenancy.team_users
        WHERE user_id = current_app_user()
    )
ORDER BY c.created_at DESC
LIMIT 100;

--! project_history : History
WITH summary AS (
    SELECT * FROM llm.chats
    WHERE id IN (SELECT MIN(id) FROM llm.chats GROUP BY conversation_id)
)
SELECT
    c.id,
    summary.prompt_id,
    CASE
        WHEN LENGTH(decrypt_text(summary.content)) > 150 THEN
            LEFT(decrypt_text(summary.content), 150) || '...'
        ELSE
            decrypt_text(summary.content)
    END AS summary,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(c.created_at)::text) as created_at_iso,
    c.created_at,
    p.prompt_type
FROM
    llm.conversations c
JOIN
    summary
ON
    c.id = summary.conversation_id
JOIN
    prompting.prompts p ON summary.prompt_id = p.id
WHERE
    c.project_id = :project_id
AND
    c.team_id IN (
        SELECT team_id
        FROM tenancy.team_users
        WHERE user_id = current_app_user()
    )
ORDER BY c.created_at DESC
LIMIT 100;
