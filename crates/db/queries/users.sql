--! user : (first_name?, last_name?)
SELECT 
    id, email, first_name, last_name, system_admin
FROM 
    auth.users
WHERE
    id = :id;
    
--! insert(first_name?, last_name?)
INSERT INTO 
    auth.users (openid_sub, email, first_name, last_name)
VALUES(:openid_sub, :email, :first_name, :last_name) 
RETURNING id;

--! user_by_openid_sub : (first_name?, last_name?)
SELECT 
    id, email, first_name, last_name, system_admin
FROM 
    auth.users
WHERE
    openid_sub = :openid_sub;

--! get_by_email : (first_name?, last_name?)
SELECT 
    id, email, first_name, last_name
FROM 
    auth.users
WHERE
    email = :email;

--! set_name(first_name, last_name, current_user_id)
UPDATE
    auth.users
SET 
    first_name = :first_name, last_name = :last_name
WHERE
    id = :current_user_id;

--! count_users
SELECT
    count(id)
FROM
    auth.users;

--! get_permissions
SELECT 
    permission
FROM 
    auth.roles_permissions
WHERE role IN (
    SELECT UNNEST(tu.roles)
    FROM tenancy.team_users tu
    WHERE tu.team_id = :team_id AND tu.user_id = current_app_user()
)
OR (
    EXISTS (
        SELECT 1
        FROM auth.users
        WHERE system_admin = true AND id = current_app_user()
    )
    AND role = 'SystemAdministrator'
);
