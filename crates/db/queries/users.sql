--! user : (first_name?, last_name?)
SELECT 
    id, email, first_name, last_name
FROM 
    users
WHERE
    id = :id;

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
WHERE
    role IN (
        SELECT 
            role 
        FROM 
            team_users tu
        WHERE tu.team_id = :team_id AND tu.user_id = current_app_user());
