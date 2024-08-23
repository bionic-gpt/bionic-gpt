-- migrate:up
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'DeleteChat';

-- migrate:down

