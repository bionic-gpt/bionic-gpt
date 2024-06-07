--: RateLimit()

--! rate_limits : RateLimit
SELECT
    l.id,
    l.api_key_id,
    l.tpm_limit,
    l.rpm_limit,
    l.created_at
FROM
    rate_limits l
ORDER BY created_at DESC;

--! new
INSERT INTO rate_limits
    (api_key_id, tpm_limit, rpm_limit)
VALUES
    (:api_key_id, :tpm_limit, :rpm_limit)
RETURNING id;

--! delete
DELETE FROM
    rate_limits
WHERE
    id = :rate_limit_id;