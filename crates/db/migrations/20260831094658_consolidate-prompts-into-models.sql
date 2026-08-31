-- migrate:up

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM model_registry.prompts
        GROUP BY model_id
        HAVING COUNT(*) > 1
    ) THEN
        RAISE EXCEPTION 'Cannot consolidate prompts: more than one prompt exists for a model';
    END IF;
END
$$;

ALTER TABLE model_registry.models
    ADD COLUMN display_name VARCHAR NOT NULL DEFAULT '',
    ADD COLUMN description VARCHAR NOT NULL DEFAULT 'Please add a description',
    ADD COLUMN disclaimer VARCHAR NOT NULL DEFAULT 'LLMs can make mistakes. Check important info.',
    ADD COLUMN example1 VARCHAR,
    ADD COLUMN example2 VARCHAR,
    ADD COLUMN example3 VARCHAR,
    ADD COLUMN example4 VARCHAR,
    ADD COLUMN system_prompt VARCHAR,
    ADD COLUMN max_history_items INT NOT NULL DEFAULT 99,
    ADD COLUMN max_completion_tokens INT,
    ADD COLUMN trim_ratio INT NOT NULL DEFAULT 80,
    ADD COLUMN temperature REAL;

-- The usage view still references the legacy chat column. Rebuild it around
-- the model id after the chat and API-key columns have been migrated.
DROP VIEW IF EXISTS inference_metrics;

UPDATE model_registry.models m
SET display_name = p.name,
    description = p.description,
    disclaimer = p.disclaimer,
    example1 = p.example1,
    example2 = p.example2,
    example3 = p.example3,
    example4 = p.example4,
    system_prompt = p.system_prompt,
    max_history_items = p.max_history_items,
    max_completion_tokens = p.max_completion_tokens,
    trim_ratio = p.trim_ratio,
    temperature = p.temperature
FROM model_registry.prompts p
WHERE p.model_id = m.id;

UPDATE model_registry.models
SET display_name = name
WHERE display_name = '';

ALTER TABLE llm.chats ADD COLUMN model_id INT;
UPDATE llm.chats c
SET model_id = p.model_id
FROM model_registry.prompts p
WHERE p.id = c.prompt_id;
ALTER TABLE llm.chats
    DROP CONSTRAINT IF EXISTS fk_prompt,
    ALTER COLUMN model_id SET NOT NULL,
    ADD CONSTRAINT chats_model_id_fkey FOREIGN KEY (model_id)
        REFERENCES model_registry.models(id) ON DELETE CASCADE,
    DROP COLUMN prompt_id;

ALTER TABLE iam.api_keys ADD COLUMN model_id INT;
UPDATE iam.api_keys a
SET model_id = p.model_id
FROM model_registry.prompts p
WHERE p.id = a.prompt_id;
ALTER TABLE iam.api_keys
    DROP CONSTRAINT IF EXISTS fk_prompt,
    ADD CONSTRAINT api_keys_model_id_fkey FOREIGN KEY (model_id)
        REFERENCES model_registry.models(id) ON DELETE CASCADE,
    DROP COLUMN prompt_id;

ALTER TABLE scheduled_tasks.tasks ADD COLUMN model_id INT;
UPDATE scheduled_tasks.tasks t
SET model_id = p.model_id
FROM model_registry.prompts p
WHERE p.id = t.prompt_id;
ALTER TABLE scheduled_tasks.tasks
    DROP CONSTRAINT IF EXISTS tasks_prompt_id_fkey,
    ALTER COLUMN model_id SET NOT NULL,
    ADD CONSTRAINT scheduled_tasks_tasks_model_id_fkey FOREIGN KEY (model_id)
        REFERENCES model_registry.models(id) ON DELETE RESTRICT,
    DROP COLUMN prompt_id;

DROP TABLE model_registry.prompt_dataset;
DROP TABLE model_registry.prompts;
DROP TYPE IF EXISTS dataset_connection;

CREATE OR REPLACE VIEW inference_metrics AS
WITH combined_data AS (
    SELECT tum.id, 'Console'::inference_type AS inference_type,
           c.model_id,
           conv.user_id,
           CASE WHEN tum.type = 'Prompt' THEN tum.tokens ELSE 0 END AS tokens_sent,
           CASE WHEN tum.type = 'Completion' THEN tum.tokens ELSE 0 END AS tokens_received,
           COALESCE(tum.duration_ms, 0) AS time_taken_ms,
           tum.created_at, tum.created_at AS updated_at
    FROM llm.token_usage_metrics tum
    JOIN llm.chats c ON tum.chat_id = c.id
    JOIN llm.conversations conv ON conv.id = c.conversation_id
    WHERE tum.chat_id IS NOT NULL
    UNION ALL
    SELECT tum.id, 'API'::inference_type AS inference_type,
           a.model_id,
           a.user_id,
           CASE WHEN tum.type = 'Prompt' THEN tum.tokens ELSE 0 END AS tokens_sent,
           CASE WHEN tum.type = 'Completion' THEN tum.tokens ELSE 0 END AS tokens_received,
           COALESCE(tum.duration_ms, 0) AS time_taken_ms,
           tum.created_at, tum.created_at AS updated_at
    FROM llm.token_usage_metrics tum
    JOIN iam.api_keys a ON a.id = tum.api_key_id
    WHERE tum.api_key_id IS NOT NULL
), recent_data AS (
    SELECT model_id, user_id, SUM(tokens_sent) AS tpm_sent,
           SUM(tokens_received) AS tpm_recv
    FROM combined_data
    WHERE created_at >= NOW() - INTERVAL '1 minute'
    GROUP BY model_id, user_id
)
SELECT model_id, user_id, tpm_sent, tpm_recv FROM recent_data;
GRANT SELECT ON inference_metrics TO application_user;
GRANT SELECT ON inference_metrics TO application_readonly;


-- migrate:down
DO $$
BEGIN
    RAISE EXCEPTION 'Consolidating prompts into models is irreversible';
END
$$;
