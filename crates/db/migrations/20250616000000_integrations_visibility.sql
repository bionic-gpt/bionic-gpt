-- migrate:up
ALTER TABLE integrations ADD COLUMN visibility visibility NOT NULL DEFAULT 'Company';
ALTER TABLE integrations ALTER COLUMN visibility DROP DEFAULT;
ALTER TABLE integrations ADD COLUMN team_id INT;
ALTER TABLE integrations ADD COLUMN created_by INT;

UPDATE integrations
SET created_by = (
    SELECT id FROM users LIMIT 1
);

UPDATE integrations
SET team_id = (
    SELECT team_id FROM team_users WHERE user_id = created_by LIMIT 1
);

ALTER TABLE integrations ALTER COLUMN team_id SET NOT NULL;
ALTER TABLE integrations ALTER COLUMN created_by SET NOT NULL;
ALTER TABLE integrations
    ADD CONSTRAINT FK_integrations_team FOREIGN KEY(team_id)
        REFERENCES teams(id) ON DELETE CASCADE;
ALTER TABLE integrations
    ADD CONSTRAINT FK_integrations_user FOREIGN KEY(created_by)
        REFERENCES users(id) ON DELETE CASCADE;

-- migrate:down
ALTER TABLE integrations DROP COLUMN visibility;
ALTER TABLE integrations DROP COLUMN created_by;
ALTER TABLE integrations DROP COLUMN team_id;
