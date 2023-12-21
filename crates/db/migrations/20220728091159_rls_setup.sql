-- migrate:up

-- Lock down the database

-- Helper functions for tenancy isolation 
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

CREATE FUNCTION is_app_user_sys_admin() RETURNS BOOLEAN AS 
$$ 
    SELECT
        system_admin
    FROM
        users
    WHERE
        id = current_app_user()
    LIMIT 1
$$ LANGUAGE SQL;
COMMENT ON FUNCTION is_app_user_sys_admin IS 
    'Is the current user a sys admin?';

CREATE FUNCTION get_teams_for_app_user() RETURNS SETOF INTEGER AS 
$$
DECLARE
    is_sys_admin BOOLEAN;
BEGIN
    is_sys_admin := is_app_user_sys_admin();

    IF is_sys_admin THEN
        RETURN QUERY SELECT
            team_id
        FROM
            team_users;
    ELSE
        RETURN QUERY SELECT
            team_id
        FROM
            team_users
        WHERE
            user_id = current_app_user();
    END IF;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;
COMMENT ON FUNCTION get_teams_for_app_user IS 
    'All the teams a user has been invited to or all teams for sys admin.';

CREATE FUNCTION get_teams_app_user_created() RETURNS setof integer AS 
$$ 
    SELECT
        id
    FROM
        teams
    WHERE
        created_by_user_id = current_app_user()
$$ LANGUAGE SQL SECURITY DEFINER;
COMMENT ON FUNCTION get_teams_app_user_created IS 
    'All the teams a user created.';

CREATE FUNCTION get_users_for_app_user() RETURNS setof integer AS 
$$ 
DECLARE
    is_sys_admin BOOLEAN;
BEGIN
    is_sys_admin := is_app_user_sys_admin();

    IF is_sys_admin THEN
        RETURN QUERY SELECT
            user_id
        FROM
            team_users;
    ELSE
        RETURN QUERY 
            SELECT
                user_id
            FROM
                team_users
            WHERE
                team_id IN (SELECT get_teams_for_app_user());
    END IF;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;
COMMENT ON FUNCTION get_users_for_app_user IS 
    'All the users from all the teams this user has been invited to.';


-- migrate:down

DROP FUNCTION current_app_user;
DROP FUNCTION get_teams_for_app_user;
DROP FUNCTION get_users_for_app_user;
DROP FUNCTION get_teams_app_user_created;
DROP FUNCTION is_app_user_sys_admin;
