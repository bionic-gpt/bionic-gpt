pub mod authz;
pub mod customer_keys;
pub mod i18n;
pub mod team_public_id;
pub mod vector_search;

use std::str::FromStr;

pub use cornucopia_async::Params;
pub use deadpool_postgres::{Pool, PoolError, Transaction};
pub use i18n::{I18n, I18nKey};
pub use queries::api_keys::ApiKey;
pub use queries::audit_trail::AuditTrail;
pub use queries::chats::Chat;
pub use queries::connections::{
    oauth2_connections_needing_refresh, update_oauth2_connection, ApiKeyConnection,
    ConnectedIntegration, Oauth2Connection, Oauth2RefreshCandidate,
};
pub use queries::conversations::{Conversation, ConversationContextSize, ConversationProject};
pub use queries::datasets::Dataset;
pub use queries::document_pipelines::DocumentPipeline;
pub use queries::generated_outputs::{GeneratedOutput, GeneratedOutputData};
pub use queries::history::History;
pub use queries::integrations::Integration;
pub use queries::invitations::{Invitation, InviteSummary};
pub use queries::models::{Model, ModelConfig};
pub use queries::oauth_clients::OauthClient;
pub use queries::object_storage::ObjectStorage;
pub use queries::openapi_specs::OpenapiSpec;
pub use queries::projects::{Project, ProjectNav, ProjectSummary};
pub use queries::prompt_flags::insert_prompt_flag;
pub use queries::providers::Provider;
pub use queries::rate_limits::RateLimit;
pub use queries::runtime_settings::RuntimeSetting;
pub use queries::scheduled_tasks::{ScheduledTask, ScheduledTaskRun};
pub use queries::skills::{Skill, SkillFile};
pub use queries::teams::GetUsers as Member;
pub use queries::teams::{Team, TeamOwner};
pub use queries::users::User;
pub use tokio_postgres::types::Json;
pub use tokio_postgres::Error as TokioPostgresError;
pub use vector_search::{get_related_context, RelatedContext};

pub fn create_pool(database_url: &str) -> deadpool_postgres::Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();
    let manager = deadpool_postgres::Manager::new(config, tokio_postgres::NoTls);

    deadpool_postgres::Pool::builder(manager).build().unwrap()
}

include!(concat!(env!("OUT_DIR"), "/cornucopia/src/lib.rs"));

pub use types::{
    AuditAccessType, AuditAction, ChatRole, ChatStatus, IntegrationType, ModelCapability,
    ModelProvider, ModelType, OpenapiSpecCategory, Permission, PromptFlagType, Role,
    ScheduledTaskRunStatus, TokenUsageType, Visibility,
};

impl std::fmt::Display for ModelProvider {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            ModelProvider::OpenAI => "OpenAI",
            ModelProvider::Groq => "Groq",
            ModelProvider::OpenRouter => "OpenRouter",
            ModelProvider::Ollama => "Ollama",
            ModelProvider::OpenAICompatible => "OpenAICompatible",
        })
    }
}

impl FromStr for ModelProvider {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "OpenAI" => Ok(Self::OpenAI),
            "Groq" => Ok(Self::Groq),
            "OpenRouter" => Ok(Self::OpenRouter),
            "Ollama" => Ok(Self::Ollama),
            "OpenAICompatible" => Ok(Self::OpenAICompatible),
            _ => Err(format!("Unsupported model provider adapter: {value}")),
        }
    }
}

#[cfg(test)]
mod migration_tests {
    const RUNTIME_SYSTEM_PROMPT_MIGRATION: &str =
        include_str!("migrations/20260819065829_runtime-system-prompt.sql");
    const DATABASE_SKILL_MIGRATION: &str =
        include_str!("migrations/20260825081847_add-database-skill.sql");
    const MODEL_PROVIDER_MIGRATION: &str =
        include_str!("migrations/20260826060507_add-model-provider-adapters.sql");
    const REMOVE_ASSISTANTS_MIGRATION: &str =
        include_str!("migrations/20260826063015_remove-assistants.sql");

    #[test]
    fn runtime_prompt_defines_the_persistent_output_contract() {
        assert!(RUNTIME_SYSTEM_PROMPT_MIGRATION
            .contains("/home/user/output — persistent workspace for generated files and state"));
        assert!(RUNTIME_SYSTEM_PROMPT_MIGRATION.contains("contents survive tool calls"));
        assert!(RUNTIME_SYSTEM_PROMPT_MIGRATION.contains(
            "Any file or state that must survive a tool call must be created under /home/user/output"
        ));
        assert!(RUNTIME_SYSTEM_PROMPT_MIGRATION.contains(
            "This includes databases, documents, spreadsheets, images, and other generated artifacts"
        ));
        assert!(RUNTIME_SYSTEM_PROMPT_MIGRATION.contains("Files created elsewhere are temporary"));
    }

    #[test]
    fn database_skill_description_matches_database_requests() {
        let expected_description = "Create, query, update, or maintain SQLite databases and structured persistent data. Use for requests involving databases, tables, records, stored state, or data that must be reused later.";

        assert!(DATABASE_SKILL_MIGRATION.contains(expected_description));
        for discovery_term in [
            "databases",
            "tables",
            "records",
            "structured persistent data",
        ] {
            assert!(expected_description.contains(discovery_term));
        }
    }

    #[test]
    fn model_provider_migration_maps_seeded_providers() {
        for mapping in [
            "WHEN 'OpenAI' THEN 'OpenAI'::model_provider",
            "WHEN 'Groq' THEN 'Groq'::model_provider",
            "WHEN 'OpenRouter' THEN 'OpenRouter'::model_provider",
            "WHEN 'Ollama (Local)' THEN 'Ollama'::model_provider",
            "ELSE 'OpenAICompatible'::model_provider",
        ] {
            assert!(MODEL_PROVIDER_MIGRATION.contains(mapping));
        }
        assert!(MODEL_PROVIDER_MIGRATION.contains("SET provider_type = provider.provider_type"));
    }

    #[test]
    fn assistant_removal_preserves_models_and_projects_in_neutral_schemas() {
        assert!(REMOVE_ASSISTANTS_MIGRATION
            .contains("DELETE FROM assistants.prompts WHERE prompt_type = 'Assistant'"));
        assert!(REMOVE_ASSISTANTS_MIGRATION
            .contains("ALTER TABLE assistants.prompts SET SCHEMA model_registry"));
        assert!(REMOVE_ASSISTANTS_MIGRATION
            .contains("ALTER TABLE assistants.projects SET SCHEMA projects"));
        assert!(REMOVE_ASSISTANTS_MIGRATION.contains("DROP SCHEMA assistants"));
        assert!(REMOVE_ASSISTANTS_MIGRATION.contains("'SetCompanyVisibility'"));
    }
}
