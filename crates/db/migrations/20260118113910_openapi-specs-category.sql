-- migrate:up
CREATE TYPE openapi_spec_category AS ENUM (
    'WebSearch',
    'Application',
    'CodeSandbox'
);

ALTER TABLE openapi_specs
ADD COLUMN category openapi_spec_category NOT NULL DEFAULT 'Application';

CREATE TABLE openapi_spec_selections (
    category openapi_spec_category PRIMARY KEY,
    openapi_spec_id INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_openapi_spec_selection FOREIGN KEY(openapi_spec_id)
        REFERENCES openapi_specs(id) ON DELETE CASCADE
);

-- Manage the updated_at column
SELECT updated_at('openapi_spec_selections');

-- Permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON openapi_spec_selections TO application_user;

GRANT SELECT ON openapi_spec_selections TO application_readonly;


-- migrate:down
DROP TABLE openapi_spec_selections;

ALTER TABLE openapi_specs DROP COLUMN category;

DROP TYPE openapi_spec_category;
