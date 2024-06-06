--: RateLimit()

--! rate_limits : RateLimit
SELECT
    l.id,
    l.api_key_id,
    l.model_id,
    l.tpm_limit,
    l.rpm_limit,
    COALESCE((SELECT name FROM models m WHERE m.id = l.model_id), 'All') as model_name,
    l.created_at
FROM
    rate_limits l
ORDER BY created_at DESC;

--! new
INSERT INTO rate_limits
    (api_key_id, model_id, tpm_limit, rpm_limit)
VALUES
    (:api_key_id, :model_id, :tpm_limit, :rpm_limit)
RETURNING id;

--! delete
DELETE FROM
    rate_limits
WHERE
    id = :rate_limit_id;