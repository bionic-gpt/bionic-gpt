-- migrate:up

CREATE TYPE model_capability AS ENUM (
  'function_calling',
  'vision',
  'tool_use'
);

CREATE TABLE model_capabilities (
  model_id INTEGER REFERENCES models(id) ON DELETE CASCADE,
  capability model_capability NOT NULL,
  value TEXT, -- optional: for non-boolean values like "parallel_calls = 5"
  PRIMARY KEY (model_id, capability)
);

GRANT SELECT, INSERT, DELETE, UPDATE ON model_capabilities TO bionic_application;
GRANT SELECT ON model_capabilities TO bionic_readonly;

-- migrate:down

DROP TABLE model_capabilities;
DROP TYPE model_capability;