-- migrate:up

GRANT DELETE ON team_users TO application_user;
GRANT DELETE ON teams TO application_user;

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

