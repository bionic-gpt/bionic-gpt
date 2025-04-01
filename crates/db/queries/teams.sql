--! team : Team(name?)
SELECT 
    id, name
FROM 
    teams
WHERE
    id = :org_id
    AND EXISTS (
        SELECT 1
        FROM team_users tu
        WHERE tu.team_id = teams.id AND tu.user_id = current_app_user()
    );
    
--! delete
DELETE FROM teams 
WHERE
    id = :org_id;

--! set_name
UPDATE
    teams
SET 
    name = :name
WHERE
    id = :org_id;

--! get_primary_team : Team(name?)
SELECT 
    id, name
FROM 
    teams
WHERE
    created_by_user_id = :created_by_user_id
ORDER BY id ASC
LIMIT 1;

--! add_user_to_team
INSERT INTO 
    team_users (user_id, team_id, roles)
VALUES(:user_id, :team_id, :roles);

--! insert_team
INSERT INTO 
    teams (created_by_user_id)
VALUES(current_app_user()) 
RETURNING id;

--! get_users : (first_name?, last_name?)
SELECT 
    u.id, 
    ou.team_id, 
    u.email, 
    u.first_name,
    u.last_name,
    ou.roles
FROM 
    team_users ou
LEFT JOIN users u ON u.id = ou.user_id
WHERE
    ou.team_id = :team_id;

--! get_teams : TeamOwner(team_name?)
SELECT 
    o.id,
    o.name as team_name, 
    u.email as team_owner
FROM 
    team_users ou
LEFT JOIN teams o ON o.id = ou.team_id
LEFT JOIN users u ON u.id = o.created_by_user_id
WHERE
    ou.user_id = :user_id
ORDER BY o.name ASC;

--! remove_user
DELETE FROM
    team_users
WHERE
    user_id = :user_id_to_remove
AND
    team_id = :team_id;