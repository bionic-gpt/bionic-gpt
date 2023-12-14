-- migrate:up

-- THIS WILL NOT TRIGGER IN THE DEVCONATINER
-- Because we have barricade as postgres user and not ft_authentication
-- To test this run
-- EXPLAIN ANALYZE INSERT INTO users (email, hashed_password) VALUES('ian@ian.com', 'dadsdsd');
CREATE OR REPLACE FUNCTION set_first_user_as_admin()
RETURNS TRIGGER AS $$
DECLARE
    user_count INTEGER;
BEGIN
    -- Count the number of users in the database
    SELECT COUNT(*) INTO user_count FROM users;

    RAISE NOTICE 'Got Users (%)', user_count;

    -- If only one user exists, set the new user as admin
    IF user_count = 1 THEN
        UPDATE users SET system_admin = TRUE;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

CREATE TRIGGER set_admin_flag
AFTER INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION set_first_user_as_admin();

-- migrate:down
DROP TRIGGER set_admin_flag ON users;
DROP FUNCTION set_first_user_as_admin;

