-- migrate:up
ALTER TABLE prompts DROP COLUMN dataset_connection;
ALTER TABLE prompts DROP COLUMN top_p;
DROP TYPE dataset_connection;

-- migrate:down
CREATE TYPE dataset_connection AS ENUM (
    'All', 
    'None', 
    'Selected'
);
COMMENT ON TYPE dataset_connection IS 'A prompt can use all datasets, no datasets or selected datasets.';
ALTER TABLE prompts ADD COLUMN dataset_connection dataset_connection NOT NULL DEFAULT 'None';
ALTER TABLE prompts ADD COLUMN top_p REAL CHECK (top_p >= 0 AND top_p <= 1);