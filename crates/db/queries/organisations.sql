--! organisation : (name?)
SELECT 
    id, name
FROM 
    organisations
WHERE
    id = :org_id;

--! set_name
UPDATE
    organisations
SET 
    name = :name
WHERE
    id = :org_id;

--! get_primary_organisation : (name?)
SELECT 
    id, name
FROM 
    organisations
WHERE
    created_by_user_id = :created_by_user_id
ORDER BY id ASC
LIMIT 1;

--! add_user_to_organisation
INSERT INTO 
    organisation_users (user_id, organisation_id, roles)
VALUES(:user_id, :organisation_id, :roles);

--! insert_organisation
INSERT INTO 
    organisations (created_by_user_id)
VALUES(current_app_user()) 
RETURNING id;

--! get_users : (first_name?, last_name?)
SELECT 
    u.id, 
    ou.organisation_id, 
    u.email, 
    u.first_name,
    u.last_name,
    ou.roles
FROM 
    organisation_users ou
LEFT JOIN users u ON u.id = ou.user_id
WHERE
    ou.organisation_id = :organisation_id;

--! get_teams : (organisation_name?)
SELECT 
    o.id,
    o.name as organisation_name, 
    u.email as team_owner
FROM 
    organisation_users ou
LEFT JOIN organisations o ON o.id = ou.organisation_id
LEFT JOIN users u ON u.id = o.created_by_user_id
WHERE
    ou.user_id = :user_id
ORDER BY o.name ASC;

--! remove_user
DELETE FROM
    organisation_users
WHERE
    user_id = :user_id_to_remove
AND
    organisation_id = :organisation_id;