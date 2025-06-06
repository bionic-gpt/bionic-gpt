-- migrate:up
CREATE TABLE token_usage_metrics (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    chat_id INT,
    api_key_id INT,
    type token_usage_type NOT NULL,
    tokens INT NOT NULL,
    duration_ms INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_chat
        FOREIGN KEY(chat_id) 
        REFERENCES chats(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_api_key
        FOREIGN KEY(api_key_id) 
        REFERENCES api_keys(id)
        ON DELETE CASCADE,

    -- Ensure either chat_id or api_key_id is set, but not both
    CONSTRAINT check_exclusive_reference 
        CHECK ((chat_id IS NOT NULL AND api_key_id IS NULL) OR 
               (chat_id IS NULL AND api_key_id IS NOT NULL))
);

COMMENT ON TABLE token_usage_metrics IS 'Unified tracking of token usage and timing for all LLM interactions';
COMMENT ON COLUMN token_usage_metrics.chat_id IS 'Reference to UI chat (mutually exclusive with api_key_id)';
COMMENT ON COLUMN token_usage_metrics.api_key_id IS 'Reference to API key for API calls (mutually exclusive with chat_id)';
COMMENT ON COLUMN token_usage_metrics.type IS 'Whether this tracks prompt tokens or completion tokens';
COMMENT ON COLUMN token_usage_metrics.tokens IS 'Number of tokens used';
COMMENT ON COLUMN token_usage_metrics.duration_ms IS 'Duration in milliseconds (only for completion type)';

-- Create indexes for performance
CREATE INDEX idx_token_usage_metrics_chat_id ON token_usage_metrics(chat_id);
CREATE INDEX idx_token_usage_metrics_api_key_id ON token_usage_metrics(api_key_id);
CREATE INDEX idx_token_usage_metrics_created_at ON token_usage_metrics(created_at);
CREATE INDEX idx_token_usage_metrics_type ON token_usage_metrics(type);

-- Grant permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON token_usage_metrics TO bionic_application;
GRANT USAGE, SELECT ON token_usage_metrics_id_seq TO bionic_application;
GRANT SELECT ON token_usage_metrics TO bionic_readonly;
GRANT SELECT ON token_usage_metrics_id_seq TO bionic_readonly;

-- migrate:down
DROP TABLE token_usage_metrics;