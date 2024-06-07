--: InferenceMetric()

--! inference_metric : InferenceMetric
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

--! inference_metrics : InferenceMetric
SELECT
    model_id,
    user_id,
    tpm_sent,
    tpm_recv
FROM
    inference_metrics;

--! inference_models
SELECT
    m.id,
    m.name AS model_name,
    COALESCE(im.user_id, 0) AS user_id,
    COALESCE(im.tpm_sent, 0) AS tpm_sent,
    COALESCE(im.tpm_recv, 0) AS tpm_recv
FROM
    models m
LEFT JOIN
    inference_metrics im
ON
    m.id = im.model_id;