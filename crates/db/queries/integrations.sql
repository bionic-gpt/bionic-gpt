--: Integration(definition?)

--! integrations : Integration
SELECT
    id,
    name,
    integration_type,
    visibility,
    definition,
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(i.created_at)::text) as created_at,
    trim(both '"' from to_json(i.updated_at)::text) as updated_at
FROM
    integrations.integrations i
WHERE
    (
        (
            i.visibility = 'Team'
            AND i.team_id IN (
                SELECT team_id
                FROM tenancy.team_users
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
    -- Convert times to ISO 8601 string.
    trim(both '"' from to_json(i.created_at)::text) as created_at,
    trim(both '"' from to_json(i.updated_at)::text) as updated_at
FROM
    integrations.integrations i
WHERE
    i.id = :model_id
    AND (
        (
            i.visibility = 'Team'
            AND i.team_id IN (
                SELECT team_id
                FROM tenancy.team_users
                WHERE user_id = current_app_user()
            )
            AND i.team_id = :team_id
        )
        OR (i.visibility = 'Company')
        OR (i.visibility = 'Private' AND i.created_by = current_app_user())
    )
ORDER BY updated_at;


--! insert(definition?)
INSERT INTO integrations.integrations (
    team_id,
    name,
    definition,
    integration_type,
    visibility,
    created_by
)
VALUES(
    :team_id,
    :name,
    :definition,
    :integration_type,
    :visibility,
    current_app_user()
)
RETURNING id;

--! update(definition?)
UPDATE
    integrations.integrations
SET
    name = :name,
    definition = :definition,
    integration_type = :integration_type,
    visibility = :visibility
WHERE
    id = :id;

--! delete
DELETE FROM
    integrations.integrations
WHERE
    id = :id;
