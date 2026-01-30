-- migrate:up
CREATE SCHEMA IF NOT EXISTS iam;
CREATE SCHEMA IF NOT EXISTS integrations;
CREATE SCHEMA IF NOT EXISTS llm;
CREATE SCHEMA IF NOT EXISTS assistants;
CREATE SCHEMA IF NOT EXISTS automation;
CREATE SCHEMA IF NOT EXISTS rag;
CREATE SCHEMA IF NOT EXISTS model_registry;
CREATE SCHEMA IF NOT EXISTS storage;
CREATE SCHEMA IF NOT EXISTS ops;

GRANT USAGE ON SCHEMA iam TO application_user;
GRANT USAGE ON SCHEMA integrations TO application_user;
GRANT USAGE ON SCHEMA llm TO application_user;
GRANT USAGE ON SCHEMA assistants TO application_user;
GRANT USAGE ON SCHEMA automation TO application_user;
GRANT USAGE ON SCHEMA rag TO application_user;
GRANT USAGE ON SCHEMA model_registry TO application_user;
GRANT USAGE ON SCHEMA storage TO application_user;
GRANT USAGE ON SCHEMA ops TO application_user;

GRANT USAGE ON SCHEMA iam TO application_readonly;
GRANT USAGE ON SCHEMA integrations TO application_readonly;
GRANT USAGE ON SCHEMA llm TO application_readonly;
GRANT USAGE ON SCHEMA assistants TO application_readonly;
GRANT USAGE ON SCHEMA automation TO application_readonly;
GRANT USAGE ON SCHEMA rag TO application_readonly;
GRANT USAGE ON SCHEMA model_registry TO application_readonly;
GRANT USAGE ON SCHEMA storage TO application_readonly;
GRANT USAGE ON SCHEMA ops TO application_readonly;

ALTER TABLE public.users SET SCHEMA iam;
ALTER TABLE public.invitations SET SCHEMA iam;
ALTER TABLE public.roles_permissions SET SCHEMA iam;
ALTER TABLE public.oauth_clients SET SCHEMA iam;
ALTER TABLE public.api_keys SET SCHEMA iam;
ALTER TABLE public.teams SET SCHEMA iam;
ALTER TABLE public.team_users SET SCHEMA iam;

ALTER TABLE public.integrations SET SCHEMA integrations;
ALTER TABLE public.oauth2_connections SET SCHEMA integrations;
ALTER TABLE public.api_key_connections SET SCHEMA integrations;
ALTER TABLE public.prompt_integration SET SCHEMA integrations;
ALTER TABLE public.openapi_specs SET SCHEMA integrations;
ALTER TABLE public.openapi_spec_selections SET SCHEMA integrations;
ALTER TABLE public.openapi_spec_api_keys SET SCHEMA integrations;

ALTER TABLE public.conversations SET SCHEMA llm;
ALTER TABLE public.chats SET SCHEMA llm;
ALTER TABLE public.api_chats SET SCHEMA llm;
ALTER TABLE public.chats_attachments SET SCHEMA llm;
ALTER TABLE public.prompt_flags SET SCHEMA llm;
ALTER TABLE public.token_usage_metrics SET SCHEMA llm;
ALTER TABLE public.rate_limits SET SCHEMA llm;

ALTER TABLE public.prompts SET SCHEMA assistants;
ALTER TABLE public.categories SET SCHEMA assistants;
ALTER TABLE public.projects SET SCHEMA assistants;
ALTER TABLE public.prompt_dataset SET SCHEMA assistants;

ALTER TABLE public.automation_cron_triggers SET SCHEMA automation;
ALTER TABLE public.automation_webhook_triggers SET SCHEMA automation;
ALTER TABLE public.automation_runs SET SCHEMA automation;

ALTER TABLE public.datasets SET SCHEMA rag;
ALTER TABLE public.document_pipelines SET SCHEMA rag;
ALTER TABLE public.documents SET SCHEMA rag;
ALTER TABLE public.chunks SET SCHEMA rag;
ALTER TABLE public.chunks_chats SET SCHEMA rag;

ALTER TABLE public.providers SET SCHEMA model_registry;
ALTER TABLE public.models SET SCHEMA model_registry;
ALTER TABLE public.model_capabilities SET SCHEMA model_registry;

ALTER TABLE public.objects SET SCHEMA storage;

ALTER TABLE public.audit_trail SET SCHEMA ops;
ALTER TABLE public.translations SET SCHEMA ops;

CREATE OR REPLACE FUNCTION is_app_user_sys_admin() RETURNS BOOLEAN AS 
$$ 
    SELECT
        system_admin
    FROM
        iam.users
    WHERE
        id = current_app_user()
    LIMIT 1
$$ LANGUAGE SQL;

CREATE OR REPLACE FUNCTION get_teams_for_app_user() RETURNS SETOF INTEGER AS 
$$
DECLARE
    is_sys_admin BOOLEAN;
BEGIN
    is_sys_admin := is_app_user_sys_admin();

    IF is_sys_admin THEN
        RETURN QUERY SELECT
            team_id
        FROM
            iam.team_users;
    ELSE
        RETURN QUERY SELECT
            team_id
        FROM
            iam.team_users
        WHERE
            user_id = current_app_user();
    END IF;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

CREATE OR REPLACE FUNCTION get_teams_app_user_created() RETURNS setof integer AS 
$$ 
    SELECT
        id
    FROM
        iam.teams
    WHERE
        created_by_user_id = current_app_user()
$$ LANGUAGE SQL SECURITY DEFINER;

CREATE OR REPLACE FUNCTION get_users_for_app_user() RETURNS setof integer AS 
$$ 
DECLARE
    is_sys_admin BOOLEAN;
BEGIN
    is_sys_admin := is_app_user_sys_admin();

    IF is_sys_admin THEN
        RETURN QUERY SELECT
            user_id
        FROM
            iam.team_users;
    ELSE
        RETURN QUERY 
            SELECT
                user_id
            FROM
                iam.team_users
            WHERE
                team_id IN (SELECT get_teams_for_app_user());
    END IF;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

CREATE OR REPLACE FUNCTION audit_by_user_and_org()
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
DECLARE
  user_id iam.users.id%type;
BEGIN
   -- trigger logic
  INSERT INTO ops.audit_trail 
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

CREATE OR REPLACE FUNCTION audit_by_user() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
DECLARE
  user_id iam.users.id%type;
BEGIN
   -- trigger logic
  INSERT INTO ops.audit_trail 
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

CREATE OR REPLACE FUNCTION audit_chats() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
DECLARE
  user_id iam.users.id%type;
  audit_id ops.audit_trail.id%type;
BEGIN
   -- Only audit when chat status changes to Success (completion)
  IF NEW.status = 'Success' AND OLD.status != 'Success' THEN
    INSERT INTO ops.audit_trail 
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

CREATE OR REPLACE FUNCTION set_first_user_as_admin()
RETURNS TRIGGER AS $$
DECLARE
    user_count INTEGER;
BEGIN
    -- Count the number of users in the database
    SELECT COUNT(*) INTO user_count FROM iam.users;

    RAISE NOTICE 'Got Users (%)', user_count;

    -- If only one user exists, set the new user as admin
    IF user_count = 1 THEN
        UPDATE iam.users SET system_admin = TRUE;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- migrate:down
ALTER TABLE iam.users SET SCHEMA public;
ALTER TABLE iam.invitations SET SCHEMA public;
ALTER TABLE iam.roles_permissions SET SCHEMA public;
ALTER TABLE iam.oauth_clients SET SCHEMA public;
ALTER TABLE iam.api_keys SET SCHEMA public;

ALTER TABLE iam.teams SET SCHEMA public;
ALTER TABLE iam.team_users SET SCHEMA public;

ALTER TABLE integrations.integrations SET SCHEMA public;
ALTER TABLE integrations.oauth2_connections SET SCHEMA public;
ALTER TABLE integrations.api_key_connections SET SCHEMA public;
ALTER TABLE integrations.prompt_integration SET SCHEMA public;
ALTER TABLE integrations.openapi_specs SET SCHEMA public;
ALTER TABLE integrations.openapi_spec_selections SET SCHEMA public;
ALTER TABLE integrations.openapi_spec_api_keys SET SCHEMA public;

ALTER TABLE llm.conversations SET SCHEMA public;
ALTER TABLE llm.chats SET SCHEMA public;
ALTER TABLE llm.api_chats SET SCHEMA public;
ALTER TABLE llm.chats_attachments SET SCHEMA public;
ALTER TABLE llm.prompt_flags SET SCHEMA public;
ALTER TABLE llm.token_usage_metrics SET SCHEMA public;
ALTER TABLE llm.rate_limits SET SCHEMA public;

ALTER TABLE assistants.prompts SET SCHEMA public;
ALTER TABLE assistants.categories SET SCHEMA public;
ALTER TABLE assistants.projects SET SCHEMA public;
ALTER TABLE assistants.prompt_dataset SET SCHEMA public;

ALTER TABLE automation.automation_cron_triggers SET SCHEMA public;
ALTER TABLE automation.automation_webhook_triggers SET SCHEMA public;
ALTER TABLE automation.automation_runs SET SCHEMA public;

ALTER TABLE rag.datasets SET SCHEMA public;
ALTER TABLE rag.document_pipelines SET SCHEMA public;
ALTER TABLE rag.documents SET SCHEMA public;
ALTER TABLE rag.chunks SET SCHEMA public;
ALTER TABLE rag.chunks_chats SET SCHEMA public;

ALTER TABLE model_registry.providers SET SCHEMA public;
ALTER TABLE model_registry.models SET SCHEMA public;
ALTER TABLE model_registry.model_capabilities SET SCHEMA public;

ALTER TABLE storage.objects SET SCHEMA public;

ALTER TABLE ops.audit_trail SET SCHEMA public;
ALTER TABLE ops.translations SET SCHEMA public;

CREATE OR REPLACE FUNCTION is_app_user_sys_admin() RETURNS BOOLEAN AS 
$$ 
    SELECT
        system_admin
    FROM
        users
    WHERE
        id = current_app_user()
    LIMIT 1
$$ LANGUAGE SQL;

CREATE OR REPLACE FUNCTION get_teams_for_app_user() RETURNS SETOF INTEGER AS 
$$
DECLARE
    is_sys_admin BOOLEAN;
BEGIN
    is_sys_admin := is_app_user_sys_admin();

    IF is_sys_admin THEN
        RETURN QUERY SELECT
            team_id
        FROM
            team_users;
    ELSE
        RETURN QUERY SELECT
            team_id
        FROM
            team_users
        WHERE
            user_id = current_app_user();
    END IF;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

CREATE OR REPLACE FUNCTION get_teams_app_user_created() RETURNS setof integer AS 
$$ 
    SELECT
        id
    FROM
        teams
    WHERE
        created_by_user_id = current_app_user()
$$ LANGUAGE SQL SECURITY DEFINER;

CREATE OR REPLACE FUNCTION get_users_for_app_user() RETURNS setof integer AS 
$$ 
DECLARE
    is_sys_admin BOOLEAN;
BEGIN
    is_sys_admin := is_app_user_sys_admin();

    IF is_sys_admin THEN
        RETURN QUERY SELECT
            user_id
        FROM
            team_users;
    ELSE
        RETURN QUERY 
            SELECT
                user_id
            FROM
                team_users
            WHERE
                team_id IN (SELECT get_teams_for_app_user());
    END IF;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

CREATE OR REPLACE FUNCTION audit_by_user_and_org()
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

CREATE OR REPLACE FUNCTION audit_by_user() 
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

CREATE OR REPLACE FUNCTION audit_chats() 
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

CREATE OR REPLACE FUNCTION set_first_user_as_admin()
RETURNS TRIGGER AS $$
DECLARE
    user_count INTEGER;
BEGIN
    -- Count the number of users in the database
    SELECT COUNT(*) INTO user_count FROM users;

    RAISE NOTICE 'Got Users (%)', user_count;

    -- If only one user exists, set the new user as admin
    IF user_count = 1 THEN
        UPDATE users SET system_admin = TRUE;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

DROP SCHEMA IF EXISTS iam;
DROP SCHEMA IF EXISTS integrations;
DROP SCHEMA IF EXISTS llm;
DROP SCHEMA IF EXISTS assistants;
DROP SCHEMA IF EXISTS automation;
DROP SCHEMA IF EXISTS rag;
DROP SCHEMA IF EXISTS model_registry;
DROP SCHEMA IF EXISTS storage;
DROP SCHEMA IF EXISTS ops;
