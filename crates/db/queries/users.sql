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

--! set_system_admin
UPDATE
    users
SET 
    system_admin = TRUE
WHERE
    id = :current_user_id;

--! is_sys_admin
SELECT
    system_admin
FROM
    users
WHERE
    id = :current_user_id;
