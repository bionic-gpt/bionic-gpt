## The Database

We use 2 main tools to manage the database

- `dbmate` For schema migrations
- `cornucopia` for generating rust code from `sql` files.

## Database Schema

<!-- mermaid-start -->
```mermaid
erDiagram
    api_chats {
        integer api_key_id FK 
        character_varying content 
        timestamp_with_time_zone created_at 
        integer id PK 
        chat_role role 
        chat_status status 
        character_varying tool_call_id 
        character_varying tool_calls 
        timestamp_with_time_zone updated_at 
    }

    api_key_connections {
        text api_key 
        timestamp_with_time_zone created_at 
        uuid external_id 
        integer id PK 
        integer integration_id FK 
        integer team_id FK 
        integer user_id FK 
        visibility visibility 
    }

    api_keys {
        character_varying api_key 
        timestamp_with_time_zone created_at 
        integer id PK 
        character_varying name 
        integer prompt_id FK 
        integer team_id FK 
        integer user_id FK 
    }

    audit_trail {
        audit_access_type access_type 
        audit_action action 
        timestamp_with_time_zone created_at 
        integer id PK 
        integer team_id FK 
        integer user_id 
    }

    automation_cron_triggers {
        timestamp_with_time_zone created_at 
        text cron_expression 
        integer id PK 
        integer prompt_id FK 
    }

    automation_runs {
        timestamp_with_time_zone completed_at 
        timestamp_with_time_zone created_at 
        integer id PK 
        integer prompt_id FK 
        timestamp_with_time_zone started_at 
        automation_run_status status 
    }

    automation_webhook_triggers {
        timestamp_with_time_zone created_at 
        integer id PK 
        integer prompt_id FK 
        text secret 
    }

    categories {
        text description 
        integer id PK 
        character_varying name UK 
    }

    chats {
        integer automation_run_id FK 
        character_varying content 
        bigint conversation_id FK 
        timestamp_with_time_zone created_at 
        integer id PK 
        integer prompt_id FK 
        chat_role role 
        chat_status status 
        character_varying tool_call_id 
        character_varying tool_calls 
        timestamp_with_time_zone updated_at 
    }

    chats_attachments {
        integer chat_id FK 
        integer object_id FK 
    }

    chunks {
        timestamp_with_time_zone created_at 
        integer document_id FK 
        vector embeddings 
        integer id PK 
        integer page_number 
        boolean processed 
        character_varying text 
        timestamp_with_time_zone updated_at 
    }

    chunks_chats {
        integer chat_id FK 
        integer chunk_id FK 
    }

    conversations {
        timestamp_with_time_zone created_at 
        bigint id PK 
        integer project_id FK 
        integer team_id FK 
        integer user_id FK 
    }

    datasets {
        chunking_strategy chunking_strategy 
        integer combine_under_n_chars 
        timestamp_with_time_zone created_at 
        integer created_by 
        integer embeddings_model_id FK 
        uuid external_id 
        integer id PK 
        boolean is_project 
        boolean multipage_sections 
        character_varying name 
        integer new_after_n_chars 
        integer team_id FK 
        timestamp_with_time_zone updated_at 
        visibility visibility 
    }

    document_pipelines {
        character_varying api_key 
        timestamp_with_time_zone created_at 
        integer dataset_id FK 
        integer id PK 
        character_varying name 
        integer team_id FK 
        timestamp_with_time_zone updated_at 
        integer user_id FK 
    }

    documents {
        bytea content 
        integer content_size 
        timestamp_with_time_zone created_at 
        integer dataset_id FK 
        character_varying failure_reason 
        character_varying file_name 
        character_varying file_type 
        integer id PK 
        integer object_id FK 
        timestamp_with_time_zone updated_at 
    }

    integrations {
        timestamp_with_time_zone created_at 
        integer created_by FK 
        jsonb definition 
        integer id PK 
        integration_type integration_type 
        character_varying name 
        integer team_id FK 
        timestamp_with_time_zone updated_at 
        visibility visibility 
    }

    invitations {
        timestamp_with_time_zone created_at 
        character_varying email 
        character_varying first_name 
        integer id PK 
        character_varying invitation_selector 
        character_varying invitation_verifier_hash 
        character_varying last_name 
        ARRAY roles 
        integer team_id FK 
    }

    model_capabilities {
        model_capability capability PK 
        integer model_id PK,FK 
        text value 
    }

    models {
        character_varying api_key 
        character_varying base_url 
        integer context_size 
        timestamp_with_time_zone created_at 
        integer id PK 
        model_type model_type 
        character_varying name 
        integer rpm_limit 
        integer tpm_limit 
        timestamp_with_time_zone updated_at 
    }

    oauth2_connections {
        text access_token 
        timestamp_with_time_zone created_at 
        timestamp_with_time_zone expires_at 
        uuid external_id 
        integer id PK 
        integer integration_id FK 
        text refresh_token 
        jsonb scopes 
        integer team_id FK 
        integer user_id FK 
        visibility visibility 
    }

    oauth_clients {
        text client_id 
        text client_secret 
        timestamp_with_time_zone created_at 
        integer id PK 
        text provider 
        text provider_url UK 
    }

    objects {
        timestamp_with_time_zone created_at 
        integer created_by FK 
        character_varying file_hash 
        character_varying file_name 
        bigint file_size 
        integer id PK 
        character_varying mime_type 
        bytea object_data 
        bytea object_data 
        character_varying object_name 
        integer team_id FK 
        timestamp_with_time_zone updated_at 
    }

    openapi_spec_api_keys {
        text api_key 
        timestamp_with_time_zone created_at 
        integer openapi_spec_id PK,FK 
        timestamp_with_time_zone updated_at 
    }

    openapi_spec_selections {
        openapi_spec_category category PK 
        timestamp_with_time_zone created_at 
        integer openapi_spec_id FK 
        timestamp_with_time_zone updated_at 
    }

    openapi_specs {
        openapi_spec_category category 
        timestamp_with_time_zone created_at 
        text description 
        integer id PK 
        boolean is_active 
        text logo_url 
        text slug UK 
        jsonb spec 
        text title 
        timestamp_with_time_zone updated_at 
    }

    projects {
        timestamp_with_time_zone created_at 
        integer created_by FK 
        integer dataset_id FK 
        integer id PK 
        text instructions 
        character_varying name 
        integer team_id FK 
        timestamp_with_time_zone updated_at 
        visibility visibility 
    }

    prompt_dataset {
        integer dataset_id FK,UK 
        integer prompt_id FK,UK 
    }

    prompt_flags {
        integer chat_id FK 
        timestamp_with_time_zone created_at 
        prompt_flag_type flag_type 
        integer id PK 
    }

    prompt_integration {
        integer api_connection_id FK 
        timestamp_with_time_zone created_at 
        integer integration_id FK,UK 
        integer oauth2_connection_id FK 
        integer prompt_id FK,UK 
    }

    prompts {
        integer category_id 
        timestamp_with_time_zone created_at 
        integer created_by 
        character_varying description 
        character_varying disclaimer 
        character_varying example1 
        character_varying example2 
        character_varying example3 
        character_varying example4 
        integer id PK 
        integer image_icon_object_id 
        integer max_chunks 
        integer max_completion_tokens 
        integer max_history_items 
        integer model_id FK 
        character_varying name 
        prompt_type prompt_type 
        character_varying system_prompt 
        integer team_id FK 
        real temperature 
        integer trim_ratio 
        timestamp_with_time_zone updated_at 
        visibility visibility 
    }

    providers {
        boolean api_key_optional 
        character_varying base_url 
        timestamp_with_time_zone created_at 
        integer default_embeddings_model_context_size 
        text default_embeddings_model_description 
        character_varying default_embeddings_model_display_name 
        character_varying default_embeddings_model_name 
        integer default_model_context_size 
        text default_model_description 
        character_varying default_model_display_name 
        character_varying default_model_name 
        integer id PK 
        character_varying name 
        text svg_logo 
        timestamp_with_time_zone updated_at 
    }

    rate_limits {
        integer api_key_id FK 
        timestamp_with_time_zone created_at 
        integer id PK 
        integer rpm_limit 
        integer tpm_limit 
    }

    roles_permissions {
        permission permission PK 
        role role PK 
    }

    schema_migrations {
        character_varying version PK 
    }

    team_users {
        ARRAY roles 
        integer team_id PK,FK 
        integer user_id PK,FK 
    }

    teams {
        integer created_by_user_id 
        integer id PK 
        character_varying name 
        text slug 
    }

    token_usage_metrics {
        integer api_key_id FK 
        integer chat_id FK 
        timestamp_with_time_zone created_at 
        integer duration_ms 
        bigint id PK 
        integer tokens 
        token_usage_type type 
    }

    translations {
        timestamp_with_time_zone created_at 
        integer id PK 
        text key UK 
        text locale UK 
        timestamp_with_time_zone updated_at 
        text value 
    }

    users {
        timestamp_with_time_zone created_at 
        character_varying email UK 
        character_varying first_name 
        integer id PK 
        character_varying last_name 
        character_varying openid_sub UK 
        boolean system_admin 
        timestamp_with_time_zone updated_at 
    }

    api_chats }o--|| api_keys : "api_key_id"
    api_key_connections }o--|| integrations : "integration_id"
    api_key_connections }o--|| teams : "team_id"
    api_key_connections }o--|| users : "user_id"
    prompt_integration }o--|| api_key_connections : "api_connection_id"
    api_keys }o--|| prompts : "prompt_id"
    api_keys }o--|| teams : "team_id"
    api_keys }o--|| users : "user_id"
    rate_limits }o--|| api_keys : "api_key_id"
    token_usage_metrics }o--|| api_keys : "api_key_id"
    audit_trail }o--|| teams : "team_id"
    automation_cron_triggers }o--|| prompts : "prompt_id"
    automation_runs }o--|| prompts : "prompt_id"
    chats }o--|| automation_runs : "automation_run_id"
    automation_webhook_triggers }o--|| prompts : "prompt_id"
    chats }o--|| conversations : "conversation_id"
    chats }o--|| prompts : "prompt_id"
    chats_attachments }o--|| chats : "chat_id"
    chunks_chats }o--|| chats : "chat_id"
    prompt_flags }o--|| chats : "chat_id"
    token_usage_metrics }o--|| chats : "chat_id"
    chats_attachments }o--|| objects : "object_id"
    chunks }o--|| documents : "document_id"
    chunks_chats }o--|| chunks : "chunk_id"
    conversations }o--|| projects : "project_id"
    conversations }o--|| teams : "team_id"
    conversations }o--|| users : "user_id"
    datasets }o--|| models : "embeddings_model_id"
    datasets }o--|| models : "model_id"
    datasets }o--|| teams : "team_id"
    document_pipelines }o--|| datasets : "dataset_id"
    documents }o--|| datasets : "dataset_id"
    projects }o--|| datasets : "dataset_id"
    prompt_dataset }o--|| datasets : "dataset_id"
    document_pipelines }o--|| teams : "team_id"
    document_pipelines }o--|| users : "user_id"
    documents }o--|| objects : "object_id"
    integrations }o--|| teams : "team_id"
    integrations }o--|| users : "created_by"
    oauth2_connections }o--|| integrations : "integration_id"
    prompt_integration }o--|| integrations : "integration_id"
    invitations }o--|| teams : "team_id"
    model_capabilities }o--|| models : "model_id"
    prompts }o--|| models : "embeddings_model_id"
    prompts }o--|| models : "model_id"
    oauth2_connections }o--|| teams : "team_id"
    oauth2_connections }o--|| users : "user_id"
    prompt_integration }o--|| oauth2_connections : "oauth2_connection_id"
    objects }o--|| teams : "team_id"
    objects }o--|| users : "created_by"
    openapi_spec_api_keys |o--|| openapi_specs : "openapi_spec_id"
    openapi_spec_selections }o--|| openapi_specs : "openapi_spec_id"
    projects }o--|| teams : "team_id"
    projects }o--|| users : "created_by"
    prompt_dataset }o--|| prompts : "prompt_id"
    prompt_integration }o--|| prompts : "prompt_id"
    prompts }o--|| teams : "team_id"
    team_users }o--|| teams : "team_id"
    team_users }o--|| users : "user_id"
```
<!-- mermaid-end -->
