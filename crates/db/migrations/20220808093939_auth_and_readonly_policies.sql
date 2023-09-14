-- migrate:up

-- Open up for authentication
CREATE POLICY authentication_policy ON users TO ft_authentication USING (true);

-- Open up for database backup
CREATE POLICY readonly_policy ON invitations FOR SELECT TO ft_readonly USING (true);
CREATE POLICY readonly_policy ON organisation_users FOR SELECT TO ft_readonly USING (true);
CREATE POLICY readonly_policy ON organisations FOR SELECT TO ft_readonly USING (true);
CREATE POLICY readonly_policy ON users FOR SELECT TO ft_readonly USING (true);

-- migrate:down


-- Drop auth policies
DROP POLICY authentication_policy ON users;

DROP POLICY readonly_policy ON invitations;
DROP POLICY readonly_policy ON organisation_users;
DROP POLICY readonly_policy ON organisations;
DROP POLICY readonly_policy ON users;
