--: CronTrigger()

--! cron_triggers_by_prompt : CronTrigger
SELECT
    id,
    prompt_id,
    cron_expression,
    trim(both '"' from to_json(created_at)::text) as created_at
FROM
    automation.automation_cron_triggers
WHERE
    prompt_id = :prompt_id
ORDER BY id;

--! insert_cron_trigger
INSERT INTO automation.automation_cron_triggers (prompt_id, cron_expression)
VALUES (:prompt_id, :cron_expression)
RETURNING id;

--! delete_cron_trigger
DELETE FROM automation.automation_cron_triggers
WHERE id = :id AND prompt_id = :prompt_id;
