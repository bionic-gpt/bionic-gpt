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
    (SELECT COUNT(id) FROM llm.conversations c WHERE c.project_id = p.id) as conversation_count,
    (SELECT COUNT(id) FROM rag.documents d WHERE d.dataset_id = p.dataset_id) as attachment_count
FROM
    prompting.projects p
WHERE
    (visibility = 'Private' AND created_by = current_app_user())
OR
    (
        visibility = 'Team'
        AND
        team_id IN (
            SELECT
                team_id
            FROM tenancy.team_users WHERE user_id = current_app_user())
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
    prompting.projects
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
                FROM tenancy.team_users WHERE user_id = current_app_user())
        )
        OR
        (visibility = 'Company')
    )
LIMIT 1;

--! insert
INSERT INTO
    prompting.projects (
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
    prompting.projects
SET
    name = :name,
    instructions = :instructions,
    visibility = :visibility
WHERE
    id = :id
AND
    team_id IN (SELECT team_id FROM tenancy.team_users WHERE user_id = current_app_user());

--! delete
DELETE FROM
    prompting.projects
WHERE
    id = :id
AND
    team_id IN (SELECT team_id FROM tenancy.team_users WHERE user_id = current_app_user());
