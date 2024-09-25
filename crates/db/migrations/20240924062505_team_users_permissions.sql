-- migrate:up

GRANT DELETE ON team_users TO bionic_application;
GRANT DELETE ON teams TO bionic_application;

ALTER TABLE invitations DROP CONSTRAINT fk_team;
ALTER TABLE invitations ADD CONSTRAINT fk_team
        FOREIGN KEY(team_id) 
        REFERENCES teams(id)
        ON DELETE CASCADE;

ALTER TABLE team_users ADD CONSTRAINT fk_team
        FOREIGN KEY(team_id) 
        REFERENCES teams(id)
        ON DELETE CASCADE;

ALTER TABLE team_users ADD CONSTRAINT fk_user
        FOREIGN KEY(user_id) 
        REFERENCES users(id)
        ON DELETE CASCADE;
-- migrate:down

