-- migrate:up
CREATE TYPE token_usage_type AS ENUM (
    'Prompt',
    'Completion'
);
COMMENT ON TYPE token_usage_type IS 'Type of token usage - either prompt tokens sent or completion tokens received';

-- migrate:down
DROP TYPE token_usage_type;