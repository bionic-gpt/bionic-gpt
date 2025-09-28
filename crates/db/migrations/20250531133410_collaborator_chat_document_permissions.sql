-- migrate:up
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ViewChats';
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ManageDocumentPipelines';

INSERT INTO roles_permissions VALUES('Collaborator', 'ViewChats');
INSERT INTO roles_permissions VALUES('Collaborator', 'ManageDocumentPipelines');

-- migrate:down
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'ViewChats';
DELETE FROM roles_permissions WHERE role = 'Collaborator' AND permission = 'ManageDocumentPipelines';
