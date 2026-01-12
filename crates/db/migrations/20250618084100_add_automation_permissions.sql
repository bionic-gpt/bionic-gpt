-- migrate:up

-- Permissions for automation_runs table
GRANT SELECT, INSERT, UPDATE, DELETE ON automation_runs TO application_user;
GRANT USAGE, SELECT ON automation_runs_id_seq TO application_user;

-- Give access to the readonly user
GRANT SELECT ON automation_runs TO application_readonly;
GRANT SELECT ON automation_runs_id_seq TO application_readonly;

-- Permissions for automation_cron_triggers table
GRANT SELECT, INSERT, UPDATE, DELETE ON automation_cron_triggers TO application_user;
GRANT USAGE, SELECT ON automation_cron_triggers_id_seq TO application_user;

-- Give access to the readonly user
GRANT SELECT ON automation_cron_triggers TO application_readonly;
GRANT SELECT ON automation_cron_triggers_id_seq TO application_readonly;

-- Permissions for automation_webhook_triggers table
GRANT SELECT, INSERT, UPDATE, DELETE ON automation_webhook_triggers TO application_user;
GRANT USAGE, SELECT ON automation_webhook_triggers_id_seq TO application_user;

-- Give access to the readonly user
GRANT SELECT ON automation_webhook_triggers TO application_readonly;
GRANT SELECT ON automation_webhook_triggers_id_seq TO application_readonly;

-- migrate:down

-- Note: No explicit permission revocation needed in down migration
-- as the tables will be dropped by the original migration's rollback