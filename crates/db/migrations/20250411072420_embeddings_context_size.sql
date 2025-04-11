-- migrate:up
UPDATE models SET context_size = 512 WHERE name = 'bge-small-en-v1.5';

-- migrate:down

