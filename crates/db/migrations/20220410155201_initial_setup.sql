-- migrate:up


-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- These roles are only created in development. In production they will
-- have already been created by the infrastructure as code and have unguessable passwords.

DO $$
BEGIN
  CREATE ROLE ft_application LOGIN ENCRYPTED PASSWORD 'testpassword';
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role ft_application -- it already exists';
END
$$;
DO $$
BEGIN
  CREATE ROLE ft_authentication LOGIN ENCRYPTED PASSWORD 'testpassword';
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role ft_authentication -- it already exists';
END
$$;
DO $$
BEGIN
  CREATE ROLE ft_readonly LOGIN ENCRYPTED PASSWORD 'testpassword';
  EXCEPTION WHEN DUPLICATE_OBJECT THEN
  RAISE NOTICE 'not creating role ft_readonly -- it already exists';
END
$$;

-- Needed so we can do backups.
GRANT SELECT ON schema_migrations TO ft_readonly;

-- migrate:down
DROP OWNED BY ft_application;
DROP OWNED BY ft_authentication;
DROP OWNED BY ft_readonly;

DROP USER ft_application;
DROP USER ft_authentication;
DROP USER ft_readonly;

