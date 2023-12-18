-- migrate:up
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ViewCurrentTeam';
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ViewPrompts';
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ManagePipelines';
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ViewDatasets';
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ManageDatasets';
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'CreateApiKeys';
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ViewAuditTrail';
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'SetupModels';
ALTER TYPE permission RENAME VALUE 'ManageTeam' TO 'InvitePeopleToTeam';

-- Team manager can see invite users
ALTER TYPE role RENAME VALUE 'Administrator' TO 'TeamManager';

ALTER TYPE role ADD VALUE IF NOT EXISTS 'SystemAdministrator';


-- migrate:down
ALTER TYPE permission RENAME VALUE 'InvitePeopleToTeam' TO 'ManageTeam';
ALTER TYPE role RENAME VALUE 'TeamManager' TO 'Administrator';