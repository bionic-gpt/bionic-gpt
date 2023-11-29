-- migrate:up

-- We make the first user that registers sys admin.
CREATE OR REPLACE FUNCTION set_system_admin()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM users) THEN
        NEW.system_admin := TRUE; -- Set system_admin to true for the first user
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create the trigger on the users table
CREATE TRIGGER check_first_user
BEFORE INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION set_system_admin();

INSERT INTO models (
    name,
    model_type,
    base_url,
    billion_parameters,
    context_size
)
VALUES(
    'bge-small-en-v1.5', 
    'Embeddings', 
    'http://embeddings-api:80/openai', 
    0, 
    0
);

INSERT INTO models (
    name,
    model_type,
    base_url,
    billion_parameters,
    context_size
)
VALUES(
    'llama-2-7b', 
    'LLM', 
    'http://llm-api:3000/v1', 
    7, 
    2048
);

-- migrate:down
DELETE FROM models;
DROP TRIGGER check_first_user ON users RESTRICT;
DROP FUNCTION set_system_admin;