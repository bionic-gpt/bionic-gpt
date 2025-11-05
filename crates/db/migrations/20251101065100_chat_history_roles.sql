-- migrate:up
INSERT INTO roles_permissions VALUES('Collaborator', 'ViewChatHistory');

-- migrate:down
DELETE FROM roles_permissions
WHERE role = 'Collaborator'
  AND permission = 'ViewChatHistory';

