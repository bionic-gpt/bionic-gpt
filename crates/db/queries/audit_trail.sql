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