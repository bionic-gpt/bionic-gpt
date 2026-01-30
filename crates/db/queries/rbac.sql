--! permissions
SELECT 
    permission
FROM 
    iam.roles_permissions
WHERE 
    role
IN 
    (
        SELECT UNNEST(roles) 
        FROM iam.team_users 
        WHERE user_id = :current_user_id AND team_id = :team_id
    );