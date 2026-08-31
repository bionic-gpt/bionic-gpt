--: Model(api_key?)
--: ModelConfig(api_key?, temperature?, max_completion_tokens?, system_prompt?, example1?, example2?, example3?, example4?, selected_datasets?, datasets?)

--! models : Model
SELECT id, name, model_type, provider_type, base_url, api_key,
       tpm_limit, rpm_limit, context_size, created_at, updated_at
FROM model_registry.models
WHERE model_type = :model_type
ORDER BY updated_at;

--! all_models : ModelConfig
SELECT id, name, model_type, provider_type, base_url, api_key,
       tpm_limit, rpm_limit, context_size, created_at, updated_at,
       display_name, description, disclaimer, example1, example2,
       example3, example4, system_prompt, max_history_items,
       max_completion_tokens, trim_ratio, temperature,
       (SELECT COALESCE(STRING_AGG(d.id::text, ','), '')
        FROM rag.datasets d
        WHERE d.visibility = 'Company'
           OR (d.visibility = 'Private' AND d.created_by = current_app_user())
           OR (d.visibility = 'Team' AND d.team_id IN
               (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user()))) AS selected_datasets,
       (SELECT COALESCE(STRING_AGG(d.name, ', '), '')
        FROM rag.datasets d
        WHERE d.visibility = 'Company'
           OR (d.visibility = 'Private' AND d.created_by = current_app_user())
           OR (d.visibility = 'Team' AND d.team_id IN
               (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user()))) AS datasets
FROM model_registry.models
ORDER BY updated_at DESC;

--! llm_models : ModelConfig
SELECT id, name, model_type, provider_type, base_url, api_key,
       tpm_limit, rpm_limit, context_size, created_at, updated_at,
       display_name, description, disclaimer, example1, example2,
       example3, example4, system_prompt, max_history_items,
       max_completion_tokens, trim_ratio, temperature,
       (SELECT COALESCE(STRING_AGG(d.id::text, ','), '')
        FROM rag.datasets d
        WHERE d.visibility = 'Company'
           OR (d.visibility = 'Private' AND d.created_by = current_app_user())
           OR (d.visibility = 'Team' AND d.team_id IN
               (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user()))) AS selected_datasets,
       (SELECT COALESCE(STRING_AGG(d.name, ', '), '')
        FROM rag.datasets d
        WHERE d.visibility = 'Company'
           OR (d.visibility = 'Private' AND d.created_by = current_app_user())
           OR (d.visibility = 'Team' AND d.team_id IN
               (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user()))) AS datasets
FROM model_registry.models
WHERE model_type = 'LLM'
ORDER BY updated_at DESC;

--! model_config : ModelConfig
SELECT id, name, model_type, provider_type, base_url, api_key,
       tpm_limit, rpm_limit, context_size, created_at, updated_at,
       display_name, description, disclaimer, example1, example2,
       example3, example4, system_prompt, max_history_items,
       max_completion_tokens, trim_ratio, temperature,
       (SELECT COALESCE(STRING_AGG(d.id::text, ','), '') FROM rag.datasets d
        WHERE d.visibility = 'Company'
           OR (d.visibility = 'Private' AND d.created_by = current_app_user())
           OR (d.visibility = 'Team' AND d.team_id IN
               (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user()))) AS selected_datasets,
       (SELECT COALESCE(STRING_AGG(d.name, ', '), '') FROM rag.datasets d
        WHERE d.visibility = 'Company'
           OR (d.visibility = 'Private' AND d.created_by = current_app_user())
           OR (d.visibility = 'Team' AND d.team_id IN
               (SELECT team_id FROM iam.team_users WHERE user_id = current_app_user()))) AS datasets
FROM model_registry.models
WHERE id = :id
LIMIT 1;

--! get_system_model : Model
SELECT id, name, model_type, provider_type, base_url, api_key,
       tpm_limit, rpm_limit, context_size, created_at, updated_at
FROM model_registry.models WHERE model_type = 'LLM'
ORDER BY created_at LIMIT 1;

--! get_system_embedding_model : Model
SELECT id, name, model_type, provider_type, base_url, api_key,
       tpm_limit, rpm_limit, context_size, created_at, updated_at
FROM model_registry.models WHERE model_type = 'Embeddings'
ORDER BY created_at LIMIT 1;

--! model : Model
SELECT id, name, model_type, provider_type, base_url, api_key,
       tpm_limit, rpm_limit, context_size, created_at, updated_at
FROM model_registry.models WHERE id = :model_id
ORDER BY updated_at;

--! model_host_by_chat_id : Model
SELECT m.id, m.name, m.model_type, m.provider_type, m.base_url, m.api_key,
       m.tpm_limit, m.rpm_limit, m.context_size, m.created_at, m.updated_at
FROM model_registry.models m
JOIN llm.chats c ON c.model_id = m.id
WHERE c.id = :chat_id
ORDER BY m.updated_at;

--! model_config_by_chat_id : ModelConfig
SELECT m.id, m.name, m.model_type, m.provider_type, m.base_url, m.api_key,
       m.tpm_limit, m.rpm_limit, m.context_size, m.created_at, m.updated_at,
       m.display_name, m.description, m.disclaimer, m.example1, m.example2,
       m.example3, m.example4, m.system_prompt, m.max_history_items,
       m.max_completion_tokens, m.trim_ratio, m.temperature,
       '' AS selected_datasets, '' AS datasets
FROM model_registry.models m
JOIN llm.chats c ON c.model_id = m.id
WHERE c.id = :chat_id
LIMIT 1;

--! insert(api_key?)
INSERT INTO model_registry.models (
    name, model_type, provider_type, base_url, api_key,
    tpm_limit, rpm_limit, context_size
)
VALUES (:name, :model_type, :provider_type, :base_url, :api_key,
        :tpm_limit, :rpm_limit, :context_size)
RETURNING id;

--! update(api_key?)
UPDATE model_registry.models SET
    name = :name, model_type = :model_type, provider_type = :provider_type,
    base_url = :base_url, api_key = :api_key, tpm_limit = :tpm_limit,
    rpm_limit = :rpm_limit, context_size = :context_size
WHERE id = :id;

--! update_config(system_prompt?, max_completion_tokens?, temperature?, example1?, example2?, example3?, example4?)
UPDATE model_registry.models SET
    display_name = :display_name, description = :description,
    disclaimer = :disclaimer, example1 = :example1, example2 = :example2,
    example3 = :example3, example4 = :example4, system_prompt = :system_prompt,
    max_history_items = :max_history_items,
    max_completion_tokens = :max_completion_tokens, trim_ratio = :trim_ratio,
    temperature = :temperature
WHERE id = :id;

--! delete
DELETE FROM model_registry.models WHERE id = :id;
