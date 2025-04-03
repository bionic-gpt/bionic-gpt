-- migrate:up
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ViewSystemPrompt';

-- migrate:down
-- Cannot remove enum values in PostgreSQL