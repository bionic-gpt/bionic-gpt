-- migrate:up

-- The first team that registers will create a prompt for the built in model
CREATE OR REPLACE FUNCTION create_prompt_for_system_model()
RETURNS TRIGGER AS $$
DECLARE
    team_count INTEGER;
    team_id INTEGER;
    category_id INTEGER;
    model_id INTEGER;
    model_name VARCHAR;
BEGIN
    -- Count the number of users in the database
    SELECT COUNT(*) INTO team_count FROM teams;

    -- If only one user exists, set the new user as admin
    IF team_count = 1 THEN

        SELECT id INTO team_id FROM teams LIMIT 1;
        SELECT id INTO model_id FROM models  WHERE model_type = 'LLM' LIMIT 1;
        SELECT name INTO model_name FROM models WHERE model_type = 'LLM' LIMIT 1;
        SELECT id INTO category_id FROM categories LIMIT 1;

        RAISE NOTICE 'Creating the default prompt for (%)', model_name;

        INSERT INTO prompts (
            team_id, 
            model_id,
            category_id, 
            name,
            visibility,
            max_history_items,
            max_chunks,
            max_tokens,
            trim_ratio,
            temperature,
            prompt_type,
            created_by
        )
        VALUES(
            team_id, 
            model_id,
            category_id, 
            model_name,
            'Company',
            3,
            10,
            1024,
            80,
            0.7,
            'Model',
            current_app_user()
        );
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- migrate:down