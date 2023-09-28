--: ApiKey()

--! api_keys : ApiKey
SELECT
    a.id,
    a.name,
    a.prompt_id,
    a.user_id,
    (SELECT name FROM prompts p WHERE p.id = a.prompt_id) as prompt_name,
    a.api_key,
    a.created_at
FROM
    api_keys a
WHERE 
    a.prompt_id IN (
        SELECT id FROM prompts WHERE model_id IN(
            SELECT id FROM models WHERE organisation_id IN(
                SELECT organisation_id 
                FROM organisation_users 
                WHERE user_id = current_app_user()
                AND organisation_id = :organisation_id
            )
        )
    )
ORDER BY created_at DESC;

--! new_api_key
INSERT INTO api_keys 
    (prompt_id, user_id, name, api_key)
VALUES
    (:prompt_id, :user_id, :name, :api_key);

--! find_api_key : ApiKey
SELECT
    a.id,
    a.name,
    a.prompt_id,
    a.user_id,
    (SELECT name FROM prompts p WHERE p.id = a.prompt_id) as prompt_name,
    a.api_key,
    a.created_at
FROM
    api_keys a
WHERE
    a.api_key = :api_key
AND
    a.prompt_id IN (
        SELECT id FROM prompts WHERE model_id IN(
            SELECT id FROM models WHERE organisation_id IN(
                SELECT organisation_id 
                FROM organisation_users 
                WHERE user_id = current_app_user()
            )
        )
    );
