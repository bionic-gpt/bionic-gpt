-- migrate:up
ALTER TYPE chat_status ADD VALUE IF NOT EXISTS 'InProgress';
COMMENT ON TYPE chat_status IS 'The status of this part of the conversation with the AI: Pending (initial), InProgress (processing), Success, Cancelled, or Error';

-- migrate:down
-- Cannot remove enum values in PostgreSQL