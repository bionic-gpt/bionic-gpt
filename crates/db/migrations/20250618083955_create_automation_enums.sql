-- migrate:up
-- Create status enum for automation run executions
CREATE TYPE automation_run_status AS ENUM (
  'Pending',
  'Running',
  'Completed',
  'Failed'
);

-- Extend prompt_type to support automations
ALTER TYPE prompt_type ADD VALUE IF NOT EXISTS 'Automation';

-- migrate:down

-- Safe to drop if nothing depends on it
DROP TYPE IF EXISTS automation_run_status;