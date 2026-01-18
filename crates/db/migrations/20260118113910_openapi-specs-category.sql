-- migrate:up
CREATE TYPE openapi_spec_category AS ENUM (
    'WebSearch',
    'Application',
    'CodeSandbox'
);

ALTER TABLE openapi_specs
ADD COLUMN category openapi_spec_category NOT NULL DEFAULT 'Application';

CREATE TABLE web_search_specs (
    id INT PRIMARY KEY DEFAULT 1,
    openapi_spec_id INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT web_search_specs_singleton CHECK (id = 1),
    CONSTRAINT FK_web_search_spec FOREIGN KEY(openapi_spec_id)
        REFERENCES openapi_specs(id) ON DELETE CASCADE
);

-- Manage the updated_at column
SELECT updated_at('web_search_specs');

-- Permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON web_search_specs TO application_user;

GRANT SELECT ON web_search_specs TO application_readonly;


-- migrate:down
DROP TABLE web_search_specs;

ALTER TABLE openapi_specs DROP COLUMN category;

DROP TYPE openapi_spec_category;
