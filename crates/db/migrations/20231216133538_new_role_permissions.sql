-- migrate:up

INSERT INTO roles_permissions VALUES('SystemAdministrator', 'ViewAuditTrail');
INSERT INTO roles_permissions VALUES('SystemAdministrator', 'SetupModels');

INSERT INTO roles_permissions VALUES('Collaborator', 'ViewCurrentTeam');
INSERT INTO roles_permissions VALUES('Collaborator', 'ViewPrompts');
INSERT INTO roles_permissions VALUES('Collaborator', 'ManageDatasets');
INSERT INTO roles_permissions VALUES('Collaborator', 'ViewDatasets');
INSERT INTO roles_permissions VALUES('Collaborator', 'CreateApiKeys');
DELETE FROM roles_permissions WHERE role = 'SystemAdministrator' AND permission = 'InvitePeopleToTeam';

-- migrate:down

DELETE FROM roles_permissions WHERE role = 'SystemAdministrator' AND permission = 'ViewAuditTrail';
DELETE FROM roles_permissions WHERE role = 'SystemAdministrator' AND permission = 'SetupModels';

DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'ViewCurrentTeam';
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'ViewPrompts';
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'ManageDatasets';
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'ViewDatasets';
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'CreateApiKeys';
DELETE FROM roles_permissions WHERE role = 'TeamManager' AND permission = 'InvitePeopleToTeam';

