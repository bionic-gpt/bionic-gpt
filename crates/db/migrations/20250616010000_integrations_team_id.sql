-- migrate:up
ALTER TABLE integrations ADD COLUMN team_id INT;

UPDATE integrations
SET team_id = (
    SELECT team_id FROM team_users WHERE user_id = created_by LIMIT 1
);

ALTER TABLE integrations ALTER COLUMN team_id SET NOT NULL;
ALTER TABLE integrations
    ADD CONSTRAINT FK_integrations_team FOREIGN KEY(team_id)
        REFERENCES teams(id) ON DELETE CASCADE;

-- migrate:down
ALTER TABLE integrations DROP CONSTRAINT IF EXISTS FK_integrations_team;
ALTER TABLE integrations DROP COLUMN team_id;
