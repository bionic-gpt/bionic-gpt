-- migrate:up

CREATE TABLE openapi_spec_api_keys (
    openapi_spec_id INT PRIMARY KEY,
    api_key TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_openapi_spec_api_keys_spec FOREIGN KEY(openapi_spec_id)
        REFERENCES openapi_specs(id) ON DELETE CASCADE
);

SELECT updated_at('openapi_spec_api_keys');

GRANT SELECT, INSERT, UPDATE, DELETE ON openapi_spec_api_keys TO application_user;
GRANT SELECT ON openapi_spec_api_keys TO application_readonly;

-- migrate:down

DROP TABLE openapi_spec_api_keys;
