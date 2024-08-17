-- migrate:up
CREATE TYPE prompt_type AS ENUM('Assistant', 'Model');

ALTER TABLE prompts ADD COLUMN prompt_type prompt_type NOT NULL DEFAULT 'Assistant';
ALTER TABLE conversations ADD COLUMN prompt_id INT;

-- Find prompts that are just there for models and update them.
UPDATE prompts SET prompt_type = 'Model' 
WHERE id NOT IN 
    (SELECT prompt_id FROM prompt_dataset)
AND
    coalesce(system_prompt, '') = '';

-- migrate:down
ALTER TABLE prompts DROP COLUMN prompt_type;
ALTER TABLE conversations DROP COLUMN prompt_id;
DROP TYPE prompt_type;