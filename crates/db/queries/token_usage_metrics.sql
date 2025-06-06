--: DailyTokenUsage()
--: DailyApiRequests()

--! create_token_usage_metric(chat_id?, api_key_id?, duration_ms?)
INSERT INTO token_usage_metrics
    (chat_id, api_key_id, type, tokens, duration_ms)
VALUES
    (:chat_id, :api_key_id, :type, :tokens, :duration_ms)
RETURNING id;

--! get_daily_token_usage_for_team : DailyTokenUsage
SELECT
    DATE(created_at) as usage_date,
    type as token_type,
    SUM(tokens) as total_tokens
FROM token_usage_metrics
WHERE api_key_id IN (SELECT id FROM api_keys WHERE team_id = :team_id)
    AND created_at >= NOW() - (:days || ' days')::INTERVAL
GROUP BY DATE(created_at), type
ORDER BY usage_date DESC;

--! get_daily_api_request_count_for_team : DailyApiRequests
SELECT
    DATE(created_at) as request_date,
    COUNT(*) as request_count
FROM token_usage_metrics
WHERE api_key_id IN (SELECT id FROM api_keys WHERE team_id = :team_id)
    AND created_at >= NOW() - (:days || ' days')::INTERVAL
GROUP BY DATE(created_at)
ORDER BY request_date DESC;

--! get_daily_token_usage_system_wide : DailyTokenUsage
SELECT
    DATE(created_at) as usage_date,
    type as token_type,
    SUM(tokens) as total_tokens
FROM token_usage_metrics
WHERE created_at >= NOW() - (:days || ' days')::INTERVAL
GROUP BY DATE(created_at), type
ORDER BY usage_date DESC;

--! get_daily_api_request_count_system_wide : DailyApiRequests
SELECT
    DATE(created_at) as request_date,
    COUNT(*) as request_count
FROM token_usage_metrics
WHERE created_at >= NOW() - (:days || ' days')::INTERVAL
GROUP BY DATE(created_at)
ORDER BY request_date DESC;