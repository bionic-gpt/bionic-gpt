--: Team(name?)
--: TeamOwner(team_name?, team_id)
--! team : Team
SELECT 
    id, name
FROM 
    iam.teams
WHERE
    id = :org_id
    AND EXISTS (
        SELECT 1
        FROM iam.team_users tu
        WHERE tu.team_id = teams.id AND tu.user_id = current_app_user()
    );
    
--! delete
DELETE FROM iam.teams 
WHERE
    id = :org_id;

--! set_name
UPDATE
    iam.teams
SET 
    name = :name
WHERE
    id = :org_id;

--! get_primary_team : Team
SELECT 
    id, name
FROM 
    iam.teams
WHERE
    created_by_user_id = :created_by_user_id
ORDER BY id ASC
LIMIT 1;

--! add_user_to_team
INSERT INTO 
    iam.team_users (user_id, team_id, roles)
VALUES(:user_id, :team_id, :roles);

--! insert_team
INSERT INTO 
    iam.teams (created_by_user_id)
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
    iam.team_users ou
LEFT JOIN iam.users u ON u.id = ou.user_id
WHERE
    ou.team_id = :team_id;

--! get_teams : TeamOwner
SELECT 
    o.id,
    o.name as team_name, 
    o.id as team_id,
    u.email as team_owner
FROM 
    iam.team_users ou
LEFT JOIN iam.teams o ON o.id = ou.team_id
LEFT JOIN iam.users u ON u.id = o.created_by_user_id
WHERE
    ou.user_id = :user_id
ORDER BY o.name ASC;

--! remove_user
DELETE FROM
    iam.team_users
WHERE
    user_id = :user_id_to_remove
AND
    team_id = :team_id;
