-- migrate:up

CREATE TABLE organisations (
    id SERIAL PRIMARY KEY, 
    name VARCHAR,
    created_by_user_id INT NOT NULL
);

COMMENT ON TABLE organisations IS 'An organisation is created for everyone that signs up. It could also have been called teams.';
COMMENT ON COLUMN organisations.name IS 'The name of the organisation i.e. Microsoft or perhaps a persons name';
COMMENT ON COLUMN organisations.created_by_user_id IS 'The action committed. i.e. deleting a secret etc.';

CREATE TABLE organisation_users (
    user_id INT NOT NULL, 
    organisation_id INT NOT NULL,
    roles role ARRAY NOT NULL,
    PRIMARY KEY (user_id, organisation_id)
);

COMMENT ON TABLE organisation_users IS 'A User can belong to multiple organisations (teams).';
COMMENT ON COLUMN organisation_users.roles IS 'The RBAC privelages the user has for this team.';

CREATE TABLE invitations (
    id SERIAL PRIMARY KEY, 
    organisation_id INT NOT NULL, 
    email VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    roles role ARRAY NOT NULL,
    invitation_selector VARCHAR NOT NULL,
    invitation_verifier_hash VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_organisation
        FOREIGN KEY(organisation_id) 
        REFERENCES organisations(id)
);

COMMENT ON TABLE invitations IS 'Invitations are generated so users can join teams (organisations)';
COMMENT ON COLUMN invitations.organisation_id IS 'The organisation that the user will join if they acccept the invite';
COMMENT ON COLUMN invitations.roles IS 'The RBAC privelages the user will receive on joining the team (organisation).';
COMMENT ON COLUMN invitations.invitation_selector IS 'To avoid timing attacks the inviation secret is split into a lookup then a verfication.';
COMMENT ON COLUMN invitations.email IS 'After we lookup the invite we check that the hash is correct';

-- Give access to the application user, the application user has no access to 
-- The sessions table and therefore cannot fake a login.
GRANT SELECT, INSERT ON organisation_users TO ft_application;
GRANT SELECT, INSERT, UPDATE ON organisations TO ft_application;
GRANT SELECT, INSERT, DELETE ON invitations TO ft_application;
GRANT USAGE, SELECT ON invitations_id_seq, organisations_id_seq TO ft_application;

-- Give access to the readonly user
GRANT SELECT ON invitations, organisations, organisation_users TO ft_readonly;
GRANT SELECT ON invitations_id_seq, organisations_id_seq TO ft_readonly;

-- migrate:down
DROP TABLE organisation_users;
DROP TABLE invitations;
DROP TABLE organisations;