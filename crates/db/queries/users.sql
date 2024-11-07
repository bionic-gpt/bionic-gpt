--! user : (first_name?, last_name?)
SELECT 
    id, email, first_name, last_name, system_admin
FROM 
    users
WHERE
    id = :id;
    
--! insert(first_name?, last_name?)
INSERT INTO 
    users (openid_sub, email, first_name, last_name)
VALUES(:openid_sub, :email, :first_name, :last_name) 
RETURNING id;

--! user_by_openid_sub : (first_name?, last_name?)
SELECT 
    id, email, first_name, last_name, system_admin
FROM 
    users
WHERE
    openid_sub = :openid_sub;

--! get_by_email : (first_name?, last_name?)
SELECT 
    id, email, first_name, last_name
FROM 
    users
WHERE
    email = :email;

--! set_name(first_name, last_name, current_user_id)
UPDATE
    users
SET 
    first_name = :first_name, last_name = :last_name
WHERE
    id = :current_user_id;

--! count_users
SELECT
    count(id)
FROM
    users;

--! get_permissions
SELECT 
    permission
FROM 
    roles_permissions
WHERE role IN (
    SELECT UNNEST(tu.roles)
    FROM team_users tu
    WHERE tu.team_id = :team_id AND tu.user_id = current_app_user()
)
OR (
    EXISTS (
        SELECT 1
        FROM users
        WHERE system_admin = true AND id = current_app_user()
    )
    AND role = 'SystemAdministrator'
);
