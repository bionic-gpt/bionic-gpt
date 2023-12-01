--! audit(id?, action?, access_type?, user_id?) : AuditTrail()
SELECT 
    id,
    COALESCE((SELECT email from users u WHERE u.id = user_id), '') as email,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(created_at)::text) as created_at,
    action, 
    access_type
FROM 
    audit_trail
WHERE 
    -- The inputs are optional in which case we can use COALESCE to skip
    id < COALESCE(:id, 2147483647)
    AND action = COALESCE(:action, action)
    AND access_type = COALESCE(:access_type, access_type)
    AND user_id = COALESCE(:user_id, user_id)
ORDER BY created_at DESC
LIMIT :limit;

--! top_users : TopUser()
SELECT
    COALESCE(u.email, '') as email,
    agg.total_tokens_sent,
    agg.name as model_name
FROM (
    SELECT
        at.user_id,
        SUM(attg.tokens_sent) AS total_tokens_sent,
        m.name
    FROM 
        audit_trail AS at
    JOIN 
        audit_trail_text_generation AS attg ON at.id = attg.audit_id
    LEFT JOIN 
        chats c ON attg.chat_id = c.id
    LEFT JOIN 
        prompts p ON c.prompt_id = p.id
    LEFT JOIN 
        models m ON p.model_id = m.id
    WHERE 
        at.created_at >= NOW() - INTERVAL '24 HOURS'
    GROUP BY 
        at.user_id, m.name
) AS agg
LEFT JOIN users u ON u.id = agg.user_id
LIMIT 10;

