-- migrate:up
CREATE FUNCTION audit_create_team() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
   -- trigger logic
  INSERT INTO audit_trail 
  (
    user_id,
    organisation_id,
    access_type,
    action,
    description
  )
  VALUES(
    NEW.user_id,
    NEW.organisation_id,
    'UserInterface',
    'CreateTeam',
    'A user has been added to the team'
  );

  RETURN NEW;
END;
$$;

CREATE FUNCTION audit_create_api_key() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
   -- trigger logic
  INSERT INTO audit_trail 
  (
    user_id,
    access_type,
    action,
    description
  )
  VALUES(
    NEW.user_id,
    'UserInterface',
    'CreateAPIKey',
    'An API key was created'
  );

  RETURN NEW;
END;
$$;

CREATE TRIGGER create_team
  AFTER INSERT
  ON organisation_users
  FOR EACH ROW
  EXECUTE PROCEDURE audit_create_team();

CREATE TRIGGER create_api_key
  AFTER INSERT
  ON api_keys
  FOR EACH ROW
  EXECUTE PROCEDURE audit_create_api_key();

-- migrate:down
DROP TRIGGER create_api_key ON api_keys RESTRICT;
DROP FUNCTION audit_create_api_key;
DROP TRIGGER create_team ON organisation_users RESTRICT;
DROP FUNCTION audit_create_team;

