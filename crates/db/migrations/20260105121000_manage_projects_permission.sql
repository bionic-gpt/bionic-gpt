-- migrate:up
INSERT INTO roles_permissions VALUES('Collaborator', 'ManageProjects');
INSERT INTO roles_permissions VALUES('TeamManager', 'ManageProjects');
INSERT INTO roles_permissions VALUES('SystemAdministrator', 'ManageProjects');

-- migrate:down
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'ManageProjects';
DELETE FROM roles_permissions WHERE role = 'TeamManager' AND permission = 'ManageProjects';
DELETE FROM roles_permissions WHERE role = 'SystemAdministrator' AND permission = 'ManageProjects';
