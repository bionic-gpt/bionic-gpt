-- migrate:up
ALTER TABLE datasets ADD COLUMN external_id uuid;
UPDATE datasets SET external_id = gen_random_uuid() WHERE external_id IS NULL;
ALTER TABLE datasets ALTER COLUMN external_id SET NOT NULL;
ALTER TABLE datasets ALTER COLUMN external_id SET DEFAULT gen_random_uuid();

-- migrate:down
ALTER TABLE datasets DROP COLUMN external_id;
