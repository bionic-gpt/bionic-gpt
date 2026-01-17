-- migrate:up
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ManageProjects';

-- migrate:down