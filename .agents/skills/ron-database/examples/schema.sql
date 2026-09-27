-- Reference schema; integrate through the destination project's migration tool.
CREATE TABLE app_users (id BIGINT PRIMARY KEY);
CREATE TABLE teams (id BIGINT PRIMARY KEY);
CREATE TABLE team_members (
    team_id BIGINT REFERENCES teams(id),
    user_id BIGINT REFERENCES app_users(id),
    PRIMARY KEY (team_id, user_id)
);
CREATE TABLE items (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    team_id BIGINT NOT NULL REFERENCES teams(id),
    owner_id BIGINT NOT NULL REFERENCES app_users(id),
    name TEXT NOT NULL CHECK (length(btrim(name)) BETWEEN 1 AND 100)
);
CREATE INDEX items_team_id_idx ON items(team_id);
