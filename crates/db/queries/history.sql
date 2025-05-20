--: History()

--! search_history : History
WITH search_results AS (
    SELECT 
        c.conversation_id,
        c.created_at,
        decrypt_text(c.user_request) as user_request,
        decrypt_text(c.response) as response
    FROM 
        chats c
    JOIN 
        conversations conv ON c.conversation_id = conv.id
    WHERE
        conv.user_id = :user_id
    AND (
        LOWER(decrypt_text(c.user_request)) LIKE LOWER('%' || :search_term || '%')
        OR 
        LOWER(decrypt_text(c.response)) LIKE LOWER('%' || :search_term || '%')
    )
)
SELECT 
    sr.conversation_id as id,
    LEFT(COALESCE(sr.user_request, sr.response), 255) as summary,
    trim(both '"' from to_json(sr.created_at)::text) as created_at_iso,
    sr.created_at
FROM 
    search_results sr
ORDER BY 
    sr.created_at DESC
LIMIT :limit;


--! history : History
WITH summary AS (
    SELECT * FROM chats
    WHERE id IN (SELECT MIN(id) FROM chats GROUP BY conversation_id)
)
SELECT 
    c.id, 
    CASE
        WHEN LENGTH(decrypt_text(summary.user_request)) > 150 THEN 
            LEFT(decrypt_text(summary.user_request), 150) || '...'
        ELSE 
            decrypt_text(summary.user_request)
    END AS summary,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(c.created_at)::text) as created_at_iso,
    c.created_at
FROM 
    conversations c
JOIN 
    summary
ON 
    c.id = summary.conversation_id
AND
    c.user_id = current_app_user()
AND
    -- Make sure the user has access to this conversation
    c.team_id IN (
        SELECT team_id 
        FROM team_users 
        WHERE user_id = current_app_user()
    )
ORDER BY c.created_at DESC
LIMIT 100;