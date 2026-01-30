--! permissions
SELECT 
    permission
FROM 
    auth.roles_permissions
WHERE 
    role
IN 
    (
        SELECT UNNEST(roles) 
        FROM tenancy.team_users 
        WHERE user_id = :current_user_id AND team_id = :team_id
    );