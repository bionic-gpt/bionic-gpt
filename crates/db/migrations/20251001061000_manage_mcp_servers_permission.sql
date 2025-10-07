-- migrate:up

ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ManageMcpKeys';

-- migrate:down
-- Enum values cannot be removed; no-op.
