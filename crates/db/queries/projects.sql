--: Project()
--: ProjectSummary(conversation_count, attachment_count)

--! projects : ProjectSummary
SELECT
    p.id,
    p.team_id,
    p.dataset_id,
    p.name,
    p.instructions,
    p.visibility,
    p.created_by,
    p.created_at,
    p.updated_at,
    (SELECT COUNT(id) FROM conversations c WHERE c.project_id = p.id) as conversation_count,
    (SELECT COUNT(id) FROM documents d WHERE d.dataset_id = p.dataset_id) as attachment_count
FROM
    projects p
WHERE
    (visibility = 'Private' AND created_by = current_app_user())
OR
    (
        visibility = 'Team'
        AND
        team_id IN (
            SELECT
                team_id
            FROM team_users WHERE user_id = current_app_user())
    )
OR
    (visibility = 'Company')
ORDER BY updated_at DESC;

--! project : Project
SELECT
    id,
    team_id,
    dataset_id,
    name,
    instructions,
    visibility,
    created_by,
    created_at,
    updated_at
FROM
    projects
WHERE
    id = :project_id
AND
    (
        (visibility = 'Private' AND created_by = current_app_user())
        OR
        (
            visibility = 'Team'
            AND
            team_id IN (
                SELECT
                    team_id
                FROM team_users WHERE user_id = current_app_user())
        )
        OR
        (visibility = 'Company')
    )
LIMIT 1;

--! insert
INSERT INTO
    projects (
        team_id,
        dataset_id,
        name,
        instructions,
        visibility,
        created_by
    )
VALUES(
    :team_id,
    :dataset_id,
    :name,
    :instructions,
    :visibility,
    current_app_user())
RETURNING id;

--! update
UPDATE
    projects
SET
    name = :name,
    instructions = :instructions,
    visibility = :visibility
WHERE
    id = :id
AND
    team_id IN (SELECT team_id FROM team_users WHERE user_id = current_app_user());

--! delete
DELETE FROM
    projects
WHERE
    id = :id
AND
    team_id IN (SELECT team_id FROM team_users WHERE user_id = current_app_user());
