-- migrate:up

ALTER TABLE scheduled_tasks.tasks
    ADD COLUMN prompt_id INT REFERENCES model_registry.prompts(id) ON DELETE RESTRICT;

UPDATE scheduled_tasks.tasks
SET prompt_id = (SELECT id FROM model_registry.prompts ORDER BY id LIMIT 1)
WHERE prompt_id IS NULL;

ALTER TABLE scheduled_tasks.tasks
    ALTER COLUMN prompt_id SET NOT NULL;


-- migrate:down

ALTER TABLE scheduled_tasks.tasks DROP COLUMN prompt_id;
