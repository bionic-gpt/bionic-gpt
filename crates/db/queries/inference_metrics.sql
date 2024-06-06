--: InferenceMetrics()

--! inference_metrics : InferenceMetrics
SELECT
    im.id,
    im.inference_type,
    im.model_id,
    im.user_id,
    im.tokens_sent,
    im.tokens_received,
    im.time_taken_ms,
    im.created_at,
    im.updated_at
FROM
    inference_metrics im
ORDER BY created_at DESC;