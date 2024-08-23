-- migrate:up
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'DeleteChat';
INSERT INTO roles_permissions VALUES('Collaborator', 'DeleteChat');

-- migrate:down

