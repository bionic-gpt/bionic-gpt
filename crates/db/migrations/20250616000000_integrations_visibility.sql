-- migrate:up
ALTER TABLE integrations ADD COLUMN visibility visibility NOT NULL DEFAULT 'Company';
ALTER TABLE integrations ALTER COLUMN visibility DROP DEFAULT;

-- migrate:down
ALTER TABLE integrations DROP COLUMN visibility;
