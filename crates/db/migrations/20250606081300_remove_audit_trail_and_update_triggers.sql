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

-- Drop the audit_trail_text_generation table
DROP TABLE IF EXISTS audit_trail_text_generation;

-- migrate:down

-- Recreate the audit_trail_text_generation table
CREATE TABLE audit_trail_text_generation (
    id int GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY, 
    audit_id INT NOT NULL, 
    chat_id INT NOT NULL, 
    tokens_sent INT NOT NULL,
    tokens_received INT NOT NULL,
    time_taken INT NOT NULL,

    CONSTRAINT fk_chat
        FOREIGN KEY(chat_id) 
        REFERENCES chats(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_audit
        FOREIGN KEY(audit_id) 
        REFERENCES audit_trail(id)
        ON DELETE CASCADE
);

COMMENT ON TABLE audit_trail_text_generation IS 'For text generation we capture extra information';

-- Grant permissions
GRANT SELECT, INSERT ON audit_trail_text_generation TO bionic_application;
GRANT USAGE, SELECT ON audit_trail_text_generation_id_seq TO bionic_application;
GRANT SELECT ON audit_trail_text_generation TO bionic_readonly;
GRANT SELECT ON audit_trail_text_generation_id_seq TO bionic_readonly;

-- Restore the original audit_chats function
DROP TRIGGER IF EXISTS update_chats ON chats RESTRICT;
DROP FUNCTION IF EXISTS audit_chats;

-- Recreate the original audit_chats function (adapted for new chats structure)
CREATE FUNCTION audit_chats() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
DECLARE
  user_id users.id%type;
  audit_id audit_trail.id%type;
BEGIN
   -- trigger logic - adapted for new structure
  IF NEW.status = 'Success' AND OLD.status != 'Success' AND NEW.role = 'Assistant' THEN
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
      COALESCE(LENGTH(NEW.content), 0),
      COALESCE(LENGTH(NEW.content), 0),
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