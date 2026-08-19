--: RuntimeSetting()

--! default_system_prompt : RuntimeSetting
SELECT
    key,
    value,
    created_at,
    updated_at
FROM
    ops.runtime_settings
WHERE
    key = 'default_system_prompt';

--! update_default_system_prompt
UPDATE
    ops.runtime_settings
SET
    value = :value
WHERE
    key = 'default_system_prompt';
