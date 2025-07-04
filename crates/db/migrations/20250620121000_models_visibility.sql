-- migrate:up
ALTER TABLE models ADD COLUMN visibility visibility NOT NULL DEFAULT 'Team';
UPDATE models SET visibility = 'Company';
ALTER TABLE models ALTER COLUMN visibility DROP DEFAULT;

-- migrate:down
ALTER TABLE models DROP COLUMN visibility;
