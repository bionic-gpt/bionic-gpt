-- migrate:up
ALTER TABLE api_key_connections ADD COLUMN external_id uuid;
UPDATE api_key_connections SET external_id = gen_random_uuid() WHERE external_id IS NULL;
ALTER TABLE api_key_connections ALTER COLUMN external_id SET NOT NULL;
ALTER TABLE api_key_connections ALTER COLUMN external_id SET DEFAULT gen_random_uuid();

ALTER TABLE oauth2_connections ADD COLUMN external_id uuid;
UPDATE oauth2_connections SET external_id = gen_random_uuid() WHERE external_id IS NULL;
ALTER TABLE oauth2_connections ALTER COLUMN external_id SET NOT NULL;
ALTER TABLE oauth2_connections ALTER COLUMN external_id SET DEFAULT gen_random_uuid();

-- migrate:down
ALTER TABLE oauth2_connections DROP COLUMN external_id;
ALTER TABLE api_key_connections DROP COLUMN external_id;
