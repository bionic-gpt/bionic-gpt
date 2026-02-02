-- migrate:up
CREATE OR REPLACE FUNCTION slugify_simple(input text)
RETURNS text
LANGUAGE sql
IMMUTABLE
AS $$
  SELECT trim(both '-' FROM
    regexp_replace(
      regexp_replace(lower(coalesce(input, '')), '[^a-z0-9]+', '-', 'g'),
      '-{2,}', '-', 'g'
    )
  );
$$;

ALTER TABLE iam.teams
  ADD COLUMN slug text;

CREATE OR REPLACE FUNCTION set_team_slug()
RETURNS TRIGGER AS $$
DECLARE
    base_slug text;
    email_prefix text;
BEGIN
    IF slugify_simple(NEW.name) = '' THEN
        SELECT split_part(email, '@', 1)
        INTO email_prefix
        FROM iam.users
        WHERE id = NEW.created_by_user_id;

        base_slug := slugify_simple(email_prefix);
        IF base_slug = '' THEN
            base_slug := 'team';
        END IF;
    ELSE
        base_slug := slugify_simple(NEW.name);
    END IF;

    NEW.slug := base_slug || '--' || NEW.id::text;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_team_slug
BEFORE INSERT OR UPDATE OF name ON iam.teams
FOR EACH ROW
EXECUTE FUNCTION set_team_slug();

UPDATE iam.teams SET name = name;

ALTER TABLE iam.teams
  ALTER COLUMN slug SET NOT NULL;

CREATE UNIQUE INDEX teams_slug_uq ON iam.teams (slug);


-- migrate:down
DROP INDEX IF EXISTS teams_slug_uq;

DROP TRIGGER IF EXISTS set_team_slug ON iam.teams;
DROP FUNCTION IF EXISTS set_team_slug() CASCADE;


DROP FUNCTION IF EXISTS slugify_simple(text);
