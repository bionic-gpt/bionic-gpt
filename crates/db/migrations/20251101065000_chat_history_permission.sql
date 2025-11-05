-- migrate:up
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ViewChatHistory';

-- migrate:down

