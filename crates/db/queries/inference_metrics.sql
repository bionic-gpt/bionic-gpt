--: InferenceMetrics()

--! inference_metrics : InferenceMetrics
SELECT
    model_id,
    user_id,
    tpm_sent,
    tpm_recv
FROM
    inference_metrics
WHERE
    model_id = :model_id
AND
    user_id = :user_id;