-- migrate:up
ALTER TABLE prompts
    RENAME COLUMN max_tokens TO max_completion_tokens;

ALTER TABLE prompts
    ALTER COLUMN max_completion_tokens DROP NOT NULL;

-- migrate:down
ALTER TABLE prompts
    RENAME COLUMN max_completion_tokens TO max_tokens;

UPDATE prompts
SET max_tokens = 1024
WHERE max_tokens IS NULL;

ALTER TABLE prompts
    ALTER COLUMN max_tokens SET NOT NULL;
