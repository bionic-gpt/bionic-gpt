--: Capability(value?)

--! get_model_capabilities : Capability
SELECT
    model_id,
    capability,
    value
FROM
    model_capabilities
WHERE
    model_id = :model_id;

--! set_model_capability
INSERT INTO model_capabilities
    (model_id, capability)
VALUES
    (:model_id, :capability)
ON CONFLICT (model_id, capability)
DO NOTHING;

--! delete_model_capability
DELETE FROM
    model_capabilities
WHERE
    model_id = :model_id
AND
    capability = :capability;

--! delete_all_model_capabilities
DELETE FROM
    model_capabilities
WHERE
    model_id = :model_id;