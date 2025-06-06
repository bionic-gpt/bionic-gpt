-- migrate:up
CREATE TABLE oauth2_connections (
    id int GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    integration_id int NOT NULL REFERENCES integrations(id) ON DELETE CASCADE,
    user_id int NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    team_id int NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    visibility visibility NOT NULL,
    access_token text NOT NULL,
    refresh_token text,
    expires_at timestamptz,
    scopes jsonb NOT NULL DEFAULT '[]'::jsonb,
    created_at timestamptz NOT NULL DEFAULT NOW()
);

-- Permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON oauth2_connections TO bionic_application;
GRANT USAGE, SELECT ON oauth2_connections_id_seq TO bionic_application;

-- migrate:down
DROP TABLE oauth2_connections;