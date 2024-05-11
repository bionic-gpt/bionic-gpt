--: RateLimit(limits_role?, user_email?, model_id?)

--! rate_limits : RateLimit
SELECT
    l.id,
    l.limits_role,
    l.user_email,
    l.model_id,
    l.tokens_per_hour,
    COALESCE((SELECT name FROM models m WHERE m.id = l.model_id), 'All') as model_name,
    l.created_at
FROM
    rate_limits l
ORDER BY created_at DESC;

--! new(limits_role?, user_email?, model_id?) 
INSERT INTO rate_limits
    (limits_role, user_email, model_id, tokens_per_hour)
VALUES
    (:limits_role, :user_email, :model_id, :tokens_per_hour)
RETURNING id;

--! delete
DELETE FROM
    rate_limits
WHERE
    id = :rate_limit_id;