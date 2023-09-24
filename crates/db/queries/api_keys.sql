--: ApiKey()

--! api_keys : ApiKey
SELECT
    id,
    name,
    prompt_id,
    api_key,
    created_at
FROM
    api_keys
WHERE 
    prompt_id IN (
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
    (prompt_id, name, api_key)
VALUES
    (:prompt_id, :name, :api_key);

--! find_api_key : ApiKey
SELECT
    id,
    name,
    prompt_id,
    api_key,
    created_at
FROM
    api_keys
WHERE
    api_key = :api_key
AND
    prompt_id IN (
        SELECT id FROM prompts WHERE model_id IN(
            SELECT id FROM models WHERE organisation_id IN(
                SELECT organisation_id 
                FROM organisation_users 
                WHERE user_id = current_app_user()
            )
        )
    );
