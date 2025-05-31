-- migrate:up

-- Assign integration permissions to roles based on access levels

-- Collaborator: Read-only access to integrations
INSERT INTO roles_permissions VALUES('Collaborator', 'ViewIntegrations');

-- TeamManager: Full integration management within their team
INSERT INTO roles_permissions VALUES('TeamManager', 'ViewIntegrations');
INSERT INTO roles_permissions VALUES('TeamManager', 'ManageIntegrations');

-- SystemAdministrator: Full access to all integration functionality
INSERT INTO roles_permissions VALUES('SystemAdministrator', 'ViewIntegrations');
INSERT INTO roles_permissions VALUES('SystemAdministrator', 'ManageIntegrations');

-- migrate:down

-- Remove role-permission mappings
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'ViewIntegrations';
DELETE FROM roles_permissions WHERE role = 'TeamManager' AND permission = 'ViewIntegrations';
DELETE FROM roles_permissions WHERE role = 'TeamManager' AND permission = 'ManageIntegrations';
DELETE FROM roles_permissions WHERE role = 'SystemAdministrator' AND permission = 'ViewIntegrations';
DELETE FROM roles_permissions WHERE role = 'SystemAdministrator' AND permission = 'ManageIntegrations';