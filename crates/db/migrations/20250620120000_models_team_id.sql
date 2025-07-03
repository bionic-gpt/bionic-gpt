-- migrate:up
-- Disable audit triggers so inserting the default team doesn't require
-- current_app_user to be set
ALTER TABLE teams DISABLE TRIGGER create_team;
ALTER TABLE teams DISABLE TRIGGER delete_team;
-- Disable the system prompt trigger as it inserts prompts using
-- current_app_user which isn't set during migrations
ALTER TABLE teams DISABLE TRIGGER set_system_prompt;

ALTER TABLE models ADD COLUMN team_id INT;
-- create a team if none exist so existing models can reference it
INSERT INTO teams (name, created_by_user_id)
SELECT 'Default Team', COALESCE((SELECT id FROM users LIMIT 1), 0)
WHERE NOT EXISTS (SELECT 1 FROM teams);
UPDATE models SET team_id = (SELECT id FROM teams ORDER BY id LIMIT 1);
ALTER TABLE models ALTER COLUMN team_id SET NOT NULL;
ALTER TABLE models
    ADD CONSTRAINT FK_models_team FOREIGN KEY(team_id)
        REFERENCES teams(id) ON DELETE CASCADE;

-- Re-enable the audit triggers
ALTER TABLE teams ENABLE TRIGGER create_team;
ALTER TABLE teams ENABLE TRIGGER delete_team;
ALTER TABLE teams ENABLE TRIGGER set_system_prompt;

-- migrate:down
ALTER TABLE models DROP CONSTRAINT FK_models_team;
ALTER TABLE models DROP COLUMN team_id;
