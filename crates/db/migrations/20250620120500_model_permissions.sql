-- migrate:up
INSERT INTO roles_permissions VALUES('Collaborator', 'SetupModels');
INSERT INTO roles_permissions VALUES('TeamManager', 'SetupModels');

-- migrate:down
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'SetupModels';
DELETE FROM roles_permissions WHERE role = 'TeamManager' AND permission = 'SetupModels';
