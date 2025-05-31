-- migrate:up

-- Add new integration permissions to the permission enum
-- ViewIntegrations: Allows users to view the list of integrations and their configurations
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ViewIntegrations';

-- ManageIntegrations: Allows users to create, update, delete integrations and manage configurations
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ManageIntegrations';

-- migrate:down

-- Note: The permission enum values cannot be removed in PostgreSQL once added
-- This is consistent with other permission migrations in the system