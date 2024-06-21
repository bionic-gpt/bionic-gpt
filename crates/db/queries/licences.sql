--! days_since_first_registration
SELECT 
    EXTRACT(DAY FROM current_timestamp - MIN(created_at))::int AS days_ago_first_user_created
FROM 
    users;


