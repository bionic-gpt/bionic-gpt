--! skills : Skill()
SELECT
    s.id,
    s.team_id,
    s.name,
    s.description,
    s.visibility,
    (SELECT COUNT(id) FROM context.skill_files WHERE skill_id = s.id) AS file_count,
    s.created_at,
    s.updated_at
FROM
    context.skills s
WHERE
    (
        (s.visibility = 'Private' AND s.created_by = current_app_user())
        OR
        (
            s.visibility = 'Team'
            AND
            s.team_id IN (
                SELECT team_id FROM iam.team_users WHERE user_id = current_app_user()
            )
        )
        OR
        (s.visibility = 'Company')
    )
ORDER BY s.updated_at DESC;

--! skill : Skill()
SELECT
    s.id,
    s.team_id,
    s.name,
    s.description,
    s.visibility,
    (SELECT COUNT(id) FROM context.skill_files WHERE skill_id = s.id) AS file_count,
    s.created_at,
    s.updated_at
FROM
    context.skills s
WHERE
    s.id = :skill_id
AND
    (
        (s.visibility = 'Private' AND s.created_by = current_app_user())
        OR
        (
            s.visibility = 'Team'
            AND
            s.team_id IN (
                SELECT team_id FROM iam.team_users WHERE user_id = current_app_user()
            )
        )
        OR
        (s.visibility = 'Company')
    );

--! visible_skill_files : SkillFile()
SELECT
    s.id AS skill_id,
    s.name AS skill_name,
    s.description,
    sf.relative_path,
    o.object_data
FROM
    context.skills s
JOIN
    context.skill_files sf ON sf.skill_id = s.id
JOIN
    storage.objects o ON o.id = sf.object_id
WHERE
    (
        (s.visibility = 'Private' AND s.created_by = current_app_user())
        OR
        (
            s.visibility = 'Team'
            AND
            s.team_id IN (
                SELECT team_id FROM iam.team_users WHERE user_id = current_app_user()
            )
        )
        OR
        (s.visibility = 'Company')
    )
ORDER BY s.name, sf.relative_path;

--! visible_skill_summaries : SkillSummary()
SELECT
    s.id AS skill_id,
    s.name AS skill_name,
    s.description
FROM
    context.skills s
WHERE
    (
        (s.visibility = 'Private' AND s.created_by = current_app_user())
        OR
        (
            s.visibility = 'Team'
            AND
            s.team_id IN (
                SELECT team_id FROM iam.team_users WHERE user_id = current_app_user()
            )
        )
        OR
        (s.visibility = 'Company')
    )
ORDER BY s.name;

--! insert_skill
INSERT INTO context.skills (
    team_id,
    name,
    description,
    visibility,
    created_by
)
VALUES (
    :team_id,
    :name,
    :description,
    :visibility,
    current_app_user()
)
RETURNING id;

--! insert_skill_file
INSERT INTO context.skill_files (
    skill_id,
    object_id,
    relative_path
)
VALUES (
    :skill_id,
    :object_id,
    :relative_path
);

--! update_skill
UPDATE
    context.skills
SET
    name = :name,
    description = :description,
    visibility = :visibility
WHERE
    id = :skill_id
AND
    team_id IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user());

--! delete_skill_files
DELETE FROM
    context.skill_files
WHERE
    skill_id = :skill_id
AND
    skill_id IN (
        SELECT id FROM context.skills
        WHERE team_id IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user())
    );

--! delete_skill
DELETE FROM
    context.skills
WHERE
    id = :skill_id
AND
    team_id IN (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user());
