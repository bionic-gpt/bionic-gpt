-- migrate:up
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'MakeAssistantPublic';

-- migrate:down

