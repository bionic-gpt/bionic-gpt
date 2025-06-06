--! create_token_usage_metric(chat_id?, api_key_id?, duration_ms?)
INSERT INTO token_usage_metrics 
    (chat_id, api_key_id, type, tokens, duration_ms)
VALUES
    (:chat_id, :api_key_id, :type, :tokens, :duration_ms)
RETURNING id;