-- migrate:up
INSERT INTO roles_permissions VALUES('Collaborator', 'DeleteChat');

-- migrate:down
DELETE FROM roles_permissions role = 'Collaborator' AND permission = 'DeleteChat';

