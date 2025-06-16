--: Integration(definition?)

--! integrations : Integration
SELECT
    id,
    name,
    integration_type,
    visibility,
    definition,
    created_at,
    updated_at
FROM
    integrations i
WHERE
    (
        (
            i.visibility = 'Team'
            AND i.team_id IN (
                SELECT team_id
                FROM team_users
                WHERE user_id = current_app_user()
            )
            AND i.team_id = :team_id
        )
        OR (i.visibility = 'Company')
        OR (i.visibility = 'Private' AND i.created_by = current_app_user())
    )
ORDER BY updated_at;

--! integration : Integration
SELECT
    id,
    name,
    integration_type,
    visibility,
    definition,
    created_at,
    updated_at
FROM
    integrations i
WHERE
    i.id = :model_id
    AND (
        (
            i.visibility = 'Team'
            AND i.team_id IN (
                SELECT team_id
                FROM team_users
                WHERE user_id = current_app_user()
            )
            AND i.team_id = :team_id
        )
        OR (i.visibility = 'Company')
        OR (i.visibility = 'Private' AND i.created_by = current_app_user())
    )
ORDER BY updated_at;


--! insert(definition?)
INSERT INTO integrations (
    name,
    definition,
    integration_type,
    visibility
)
VALUES(
    :name,
    :definition,
    :integration_type,
    :visibility
)
RETURNING id;

--! update(definition?)
UPDATE
    integrations
SET
    name = :name,
    definition = :definition,
    integration_type = :integration_type,
    visibility = :visibility
WHERE
    id = :id;

--! delete
DELETE FROM
    integrations
WHERE
    id = :id;
