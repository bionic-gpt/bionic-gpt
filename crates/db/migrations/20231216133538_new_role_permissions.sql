-- migrate:up

INSERT INTO roles_permissions VALUES('Administrator', 'ViewAuditTrail');
INSERT INTO roles_permissions VALUES('Administrator', 'SetupModels');

INSERT INTO roles_permissions VALUES('Collaborator', 'ViewCurrentTeam');
INSERT INTO roles_permissions VALUES('Collaborator', 'ViewPrompts');
INSERT INTO roles_permissions VALUES('Collaborator', 'ManageDataset');
INSERT INTO roles_permissions VALUES('Collaborator', 'ViewDataset');
INSERT INTO roles_permissions VALUES('Collaborator', 'CreateApiKeys');
INSERT INTO roles_permissions VALUES('TeamManager', 'InvitePeopleToTeam');
DELETE FROM roles_permissions WHERE role = 'Administrator' AND permission = 'InvitePeopleToTeam';

-- migrate:down

DELETE FROM roles_permissions WHERE role = 'Administrator' AND permission = 'ViewAuditTrail';
DELETE FROM roles_permissions WHERE role = 'Administrator' AND permission = 'SetupModels';

DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'ViewCurrentTeam';
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'ViewPrompts';
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'ManageDatasets';
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'ViewDatasets';
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'CreateApiKeys';

DELETE FROM roles_permissions WHERE role = 'TeamManager' AND permission = 'InvitePeopleToTeam';

