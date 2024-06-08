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
    COALESCE(SUM(im.tpm_sent), 0)::int AS tpm_sent,
    COALESCE(SUM(im.tpm_recv), 0)::int AS tpm_recv
FROM
    models m
LEFT JOIN
    inference_metrics im
ON
    m.id = im.model_id
GROUP BY
    m.id, m.name;