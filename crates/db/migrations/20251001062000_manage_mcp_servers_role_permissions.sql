-- migrate:up

INSERT INTO roles_permissions VALUES('TeamManager', 'ManageMcpKeys');
INSERT INTO roles_permissions VALUES('SystemAdministrator', 'ManageMcpKeys');

-- migrate:down

DELETE FROM roles_permissions WHERE role = 'TeamManager' AND permission = 'ManageMcpKeys';
DELETE FROM roles_permissions WHERE role = 'SystemAdministrator' AND permission = 'ManageMcpKeys';
