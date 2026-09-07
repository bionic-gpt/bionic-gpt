-- migrate:up
ALTER TABLE model_registry.models
    ADD COLUMN reasoning_effort TEXT
    CHECK (reasoning_effort IN ('none', 'minimal', 'low', 'medium', 'high', 'xhigh', 'max'));

-- migrate:down
ALTER TABLE model_registry.models DROP COLUMN reasoning_effort;
