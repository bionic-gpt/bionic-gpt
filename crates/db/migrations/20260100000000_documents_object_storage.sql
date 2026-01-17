-- migrate:up
ALTER TABLE documents
    ADD COLUMN object_id INT REFERENCES objects(id) ON DELETE SET NULL;

ALTER TABLE documents
    ALTER COLUMN content DROP NOT NULL;

-- migrate:down
ALTER TABLE documents
    ALTER COLUMN content SET NOT NULL;

ALTER TABLE documents
    DROP COLUMN object_id;
