-- migrate:up
ALTER TABLE rate_limits DROP COLUMN limits_role;
ALTER TABLE rate_limits DROP COLUMN tokens_per_hour;
ALTER TABLE rate_limits ADD COLUMN tpm_limit INT NOT NULL;
ALTER TABLE rate_limits ADD COLUMN rpm_limit INT NOT NULL;

ALTER TABLE models DROP COLUMN billion_parameters;
ALTER TABLE models ADD COLUMN tpm_limit INT NOT NULL DEFAULT 10000;
ALTER TABLE models ADD COLUMN rpm_limit INT NOT NULL DEFAULT 10000;


DROP TABLE rate_limits;

CREATE TABLE rate_limits (
    id int GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    api_key_id INT,
    model_id INT NOT NULL,
    tpm_limit INT NOT NULL,
    rpm_limit INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_model FOREIGN KEY(model_id)
        REFERENCES models(id) ON DELETE CASCADE
);

CREATE TYPE inference_type AS ENUM (
    'API',
    'Console'
);

CREATE TABLE inference_metrics (
    id int GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY, 
    input_tokens INT NOT NULL,
    output_tokens INT NOT NULL,
    time_to_first_token INT NOT NULL,
    model_id INT,
    user_id INT NOT NULL,
    inference_type inference_type NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_model FOREIGN KEY(model_id)
        REFERENCES models(id) ON DELETE CASCADE,

    CONSTRAINT FK_user FOREIGN KEY(user_id)
        REFERENCES users(id) ON DELETE CASCADE
);

-- Give access to the application user.
GRANT SELECT, INSERT, UPDATE, DELETE ON inference_metrics TO bionic_application;
GRANT USAGE, SELECT ON inference_metrics_id_seq TO bionic_application;

-- Give access to the readonly user
GRANT SELECT ON inference_metrics TO bionic_readonly;
GRANT SELECT ON inference_metrics_id_seq TO bionic_readonly;

-- Give access to the application user.
GRANT SELECT, INSERT, UPDATE, DELETE ON rate_limits TO bionic_application;
GRANT USAGE, SELECT ON rate_limits_id_seq TO bionic_application;

-- Give access to the readonly user
GRANT SELECT ON rate_limits TO bionic_readonly;
GRANT SELECT ON rate_limits_id_seq TO bionic_readonly;

-- migrate:down

DROP TABLE inference_metrics;
DROP TYPE inference_type;

DROP TABLE rate_limits;

CREATE TABLE rate_limits (
    id int GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    limits_role VARCHAR,
    user_email VARCHAR,
    model_id INT,
    tokens_per_hour INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT FK_model FOREIGN KEY(model_id)
        REFERENCES models(id) ON DELETE CASCADE
);

ALTER TABLE models ADD COLUMN billion_parameters INT NOT NULL DEFAULT 0;
ALTER TABLE models DROP COLUMN tpm_limit;
ALTER TABLE models DROP COLUMN rpm_limit;