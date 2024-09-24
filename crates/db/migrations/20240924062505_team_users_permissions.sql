-- migrate:up

GRANT DELETE ON team_users TO bionic_application;
GRANT DELETE ON teams TO bionic_application;

-- migrate:down

