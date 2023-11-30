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
    COALESCE((SELECT email from users u WHERE u.id = at.user_id), '') as email, 
    SUM(attg.tokens_sent) AS total_tokens_sent,
    (SELECT name FROM models m WHERE m.id IN (SELECT model_id FROM prompts p WHERE p.id IN (SELECT prompt_id FROM chats c WHERE c.id = attg.chat_id))) as model_name
FROM 
    audit_trail AS at
JOIN 
    audit_trail_text_generation AS attg ON at.id = attg.audit_id
WHERE 
    at.created_at >= NOW() - INTERVAL '24 HOURS'  -- Filter for the last 24 hours
GROUP BY 
    at.user_id, attg.chat_id
ORDER BY 
    total_tokens_sent DESC
LIMIT 10;