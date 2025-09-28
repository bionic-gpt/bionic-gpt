-- migrate:up
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ViewChats';
ALTER TYPE permission ADD VALUE IF NOT EXISTS 'ManageDocumentPipelines';

-- migrate:down
