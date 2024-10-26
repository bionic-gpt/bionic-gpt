-- migrate:up
INSERT INTO roles_permissions VALUES('Collaborator', 'MakeAssistantPublic');

-- migrate:down
DELETE FROM roles_permissions 
WHERE role = 'Collaborator' AND permission = 'MakeAssistantPublic';
