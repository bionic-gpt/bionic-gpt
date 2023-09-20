-- migrate:up

-- Lock down the database
ALTER TABLE invitations ENABLE ROW LEVEL SECURITY;
ALTER TABLE organisation_users ENABLE ROW LEVEL SECURITY;
ALTER TABLE organisations ENABLE ROW LEVEL SECURITY;
ALTER TABLE users ENABLE ROW LEVEL SECURITY;

-- Helper functions for tenacny isolation 
CREATE FUNCTION current_app_user() RETURNS INTEGER AS 
$$ 
    SELECT
        current_setting(
            'row_level_security.user_id',
            false
        )::integer 
$$ LANGUAGE SQL;
COMMENT ON FUNCTION current_app_user IS 
    'These needs to be set by the application before accessing the database.';

CREATE FUNCTION get_orgs_for_app_user() RETURNS setof integer AS 
$$
    SELECT
        organisation_id
    FROM
        organisation_users
    WHERE
        user_id = current_app_user()
$$ LANGUAGE SQL SECURITY DEFINER;
COMMENT ON FUNCTION get_orgs_for_app_user IS 
    'All the orgs a user has been invited to.';

CREATE FUNCTION get_orgs_app_user_created() RETURNS setof integer AS 
$$ 
    SELECT
        id
    FROM
        organisations
    WHERE
        created_by_user_id = current_app_user()
$$ LANGUAGE SQL SECURITY DEFINER;
COMMENT ON FUNCTION get_orgs_app_user_created IS 
    'All the orgs a user created.';

CREATE FUNCTION get_users_for_app_user() RETURNS setof integer AS 
$$ 
    SELECT
        user_id
    FROM
        organisation_users
    WHERE
        organisation_id IN (SELECT get_orgs_for_app_user())
$$ LANGUAGE SQL SECURITY DEFINER;
COMMENT ON FUNCTION get_users_for_app_user IS 
    'All the users from all the orgs this user has been invited to.';


-- migrate:down
ALTER TABLE invitations DISABLE ROW LEVEL SECURITY;
ALTER TABLE organisation_users DISABLE ROW LEVEL SECURITY;
ALTER TABLE organisations DISABLE ROW LEVEL SECURITY;
ALTER TABLE users DISABLE ROW LEVEL SECURITY;

DROP FUNCTION current_app_user;
DROP FUNCTION get_orgs_for_app_user;
DROP FUNCTION get_users_for_app_user;
DROP FUNCTION get_orgs_app_user_created;
