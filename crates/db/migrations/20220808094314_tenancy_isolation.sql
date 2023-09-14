-- migrate:up


CREATE POLICY multi_tenancy_policy ON invitations FOR ALL TO ft_application
USING (
    -- Is this invitation from an org we have access to?
    organisation_id IN (SELECT get_orgs_for_app_user())
    -- If the invite is not accepted yet, then we check against the users email address.
    OR (
        email IN (
            SELECT email FROM users WHERE id = current_app_user()
        )
    )
)
WITH CHECK (
    -- Is this invitation from an org we have access to?
    organisation_id IN (SELECT get_orgs_for_app_user())
);

-- organisation_users
CREATE POLICY multi_tenancy_policy_insert ON organisation_users FOR INSERT TO ft_application
WITH CHECK (
    organisation_id IN (
        SELECT organisation_id FROM invitations 
    )
    OR 
    organisation_id IN (
        SELECT get_orgs_app_user_created()
    )
);

CREATE POLICY multi_tenancy_policy_select ON organisation_users FOR SELECT TO ft_application
USING (
    organisation_id IN (SELECT get_orgs_for_app_user())
);

CREATE POLICY multi_tenancy_policy_delete ON organisation_users FOR DELETE TO ft_application
USING (
    organisation_id IN (SELECT get_orgs_for_app_user())
);

CREATE POLICY multi_tenancy_policy ON organisations FOR ALL TO ft_application
USING (
    id IN (SELECT get_orgs_for_app_user())
    OR
    created_by_user_id = current_app_user()
);

CREATE POLICY multi_tenancy_policy ON users FOR ALL TO ft_application
USING (id IN (SELECT get_users_for_app_user()));

COMMENT ON POLICY multi_tenancy_policy ON invitations IS 
    'A users can access inviation from one of the orgs or if it has the same email address';
COMMENT ON POLICY multi_tenancy_policy_insert ON organisation_users IS 
    'A user on connect users with orgs it created or where an invite exists.';
COMMENT ON POLICY multi_tenancy_policy_select ON organisation_users IS 
    'Only disconnect a user from an org if we have access to that org.';
COMMENT ON POLICY multi_tenancy_policy_select ON organisation_users IS 
    'Allow the user to see the organisation-users table';
COMMENT ON POLICY multi_tenancy_policy ON organisations IS 
    'A user can see all the orgs they have created or been invited to.';
COMMENT ON POLICY multi_tenancy_policy ON users IS 
    'A user can see all the users for orgs they have created or been invited to.';

-- migrate:down
DROP POLICY multi_tenancy_policy ON invitations;
DROP POLICY multi_tenancy_policy ON organisations;
DROP POLICY multi_tenancy_policy_insert ON organisation_users;
DROP POLICY multi_tenancy_policy_select ON organisation_users;
DROP POLICY multi_tenancy_policy_delete ON organisation_users;
DROP POLICY multi_tenancy_policy ON users;
