-- migrate:up
UPDATE prompts
SET max_completion_tokens = NULL
WHERE prompt_type = 'Model';


-- migrate:down
