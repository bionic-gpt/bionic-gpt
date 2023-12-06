-- migrate:up
CREATE FUNCTION audit_by_user_and_org()
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
DECLARE
  user_id users.id%type;
BEGIN
   -- trigger logic
  INSERT INTO audit_trail 
  (
    user_id,
    team_id,
    access_type,
    action
  )
  VALUES(
    current_app_user(),
    NEW.team_id,
    TG_ARGV[0]::audit_access_type,
    TG_ARGV[1]::audit_action
  );

  RETURN NEW;
END;
$$;

CREATE FUNCTION audit_by_user() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
DECLARE
  user_id users.id%type;
BEGIN
   -- trigger logic
  INSERT INTO audit_trail 
  (
    user_id,
    access_type,
    action
  )
  VALUES(
    current_app_user(),
    TG_ARGV[0]::audit_access_type,
    TG_ARGV[1]::audit_action
  );

  RETURN NEW;
END;
$$;

CREATE FUNCTION audit_chats() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
DECLARE
  user_id users.id%type;
  audit_id audit_trail.id%type;
BEGIN
   -- trigger logic
  IF NEW.response IS NOT NULL THEN
    INSERT INTO audit_trail 
    (
      user_id,
      access_type,
      action
    )
    VALUES(
      current_app_user(),
      TG_ARGV[0]::audit_access_type,
      'TextGeneration'
    )
    RETURNING id INTO audit_id;

    INSERT INTO audit_trail_text_generation (
      audit_id, 
      chat_id, 
      tokens_sent,
      tokens_received,
      time_taken
    ) VALUES (
      audit_id,
      NEW.id,
      LENGTH(NEW.prompt),
      LENGTH(NEW.response),
      EXTRACT (EPOCH FROM (NEW.updated_at - NEW.created_at))
    );
  END IF;

  RETURN NEW;
END;
$$;

CREATE TRIGGER update_chats
  AFTER UPDATE
  ON chats
  FOR EACH ROW
  EXECUTE PROCEDURE audit_chats('UserInterface');

CREATE TRIGGER create_member
  AFTER INSERT
  ON team_users
  FOR EACH ROW
  EXECUTE PROCEDURE audit_by_user_and_org('UserInterface', 'CreateMember');

CREATE TRIGGER delete_member
  AFTER DELETE
  ON team_users
  FOR EACH ROW
  EXECUTE PROCEDURE audit_by_user_and_org('UserInterface', 'DeleteMember');

CREATE TRIGGER create_invite
  AFTER INSERT
  ON invitations
  FOR EACH ROW
  EXECUTE PROCEDURE audit_by_user_and_org('UserInterface', 'CreateInvite');

CREATE TRIGGER create_team
  AFTER INSERT
  ON teams
  FOR EACH ROW
  EXECUTE PROCEDURE audit_by_user('UserInterface', 'CreateTeam');

CREATE TRIGGER delete_team
  AFTER DELETE
  ON teams
  FOR EACH ROW
  EXECUTE PROCEDURE audit_by_user('UserInterface', 'DeleteTeam');

CREATE TRIGGER create_api_key
  AFTER INSERT
  ON api_keys
  FOR EACH ROW
  EXECUTE PROCEDURE audit_by_user_and_org('UserInterface', 'CreateAPIKey');

CREATE TRIGGER revoke_api_key
  AFTER DELETE
  ON api_keys
  FOR EACH ROW
  EXECUTE PROCEDURE audit_by_user_and_org('UserInterface', 'DeleteAPIKey');

CREATE TRIGGER create_pipeline_key
  AFTER INSERT
  ON document_pipelines
  FOR EACH ROW
  EXECUTE PROCEDURE audit_by_user_and_org('UserInterface', 'CreatePipelineKey');

CREATE TRIGGER revoke_pipeline_key
  AFTER DELETE
  ON document_pipelines
  FOR EACH ROW
  EXECUTE PROCEDURE audit_by_user_and_org('UserInterface', 'DeletePipelineKey');

-- migrate:down

DROP TRIGGER update_chats ON chats RESTRICT;
DROP TRIGGER create_team ON teams RESTRICT;
DROP TRIGGER delete_team ON teams RESTRICT;
DROP TRIGGER delete_member ON team_users RESTRICT;
DROP TRIGGER create_member ON team_users RESTRICT;
DROP TRIGGER create_invite ON invitations RESTRICT;
DROP TRIGGER create_api_key ON api_keys RESTRICT;
DROP TRIGGER revoke_api_key ON api_keys RESTRICT;
DROP TRIGGER create_pipeline_key ON document_pipelines RESTRICT;
DROP TRIGGER revoke_pipeline_key ON document_pipelines RESTRICT;
DROP FUNCTION audit_by_user_and_org;
DROP FUNCTION audit_by_user;
DROP FUNCTION audit_chats;

