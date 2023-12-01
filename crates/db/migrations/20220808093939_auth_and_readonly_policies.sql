-- migrate:up

-- Open up for authentication
CREATE POLICY authentication_policy ON users TO ft_authentication USING (true);

-- Open up for database backup
CREATE POLICY readonly_policy ON invitations FOR SELECT TO ft_readonly USING (true);
CREATE POLICY readonly_policy ON team_users FOR SELECT TO ft_readonly USING (true);
CREATE POLICY readonly_policy ON teams FOR SELECT TO ft_readonly USING (true);
CREATE POLICY readonly_policy ON users FOR SELECT TO ft_readonly USING (true);

-- migrate:down


-- Drop auth policies
DROP POLICY authentication_policy ON users;

DROP POLICY readonly_policy ON invitations;
DROP POLICY readonly_policy ON team_users;
DROP POLICY readonly_policy ON teams;
DROP POLICY readonly_policy ON users;
