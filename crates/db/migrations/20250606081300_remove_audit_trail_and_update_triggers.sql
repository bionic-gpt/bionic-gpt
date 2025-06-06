-- migrate:up

-- Drop the trigger that references audit_trail_text_generation
DROP TRIGGER IF EXISTS update_chats ON chats RESTRICT;

-- Drop and recreate the audit_chats function to remove audit_trail_text_generation references
DROP FUNCTION IF EXISTS audit_chats;

-- Create a simplified audit_chats function that doesn't use audit_trail_text_generation
CREATE FUNCTION audit_chats() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
DECLARE
  user_id users.id%type;
  audit_id audit_trail.id%type;
BEGIN
   -- Only audit when chat status changes to Success (completion)
  IF NEW.status = 'Success' AND OLD.status != 'Success' THEN
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
    );
  END IF;

  RETURN NEW;
END;
$$;

-- Recreate the trigger with the updated function
CREATE TRIGGER update_chats
  AFTER UPDATE
  ON chats
  FOR EACH ROW
  EXECUTE PROCEDURE audit_chats('UserInterface');

-- The audit_trail_text_generation table was already dropped in the previous migration
-- So we don't need to drop it again, but we should verify it's gone

-- migrate:down

-- Restore the original audit_chats function
DROP TRIGGER IF EXISTS update_chats ON chats RESTRICT;
DROP FUNCTION IF EXISTS audit_chats;

-- Recreate the original audit_chats function (but this will fail if audit_trail_text_generation doesn't exist)
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