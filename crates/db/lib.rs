pub mod authz;
pub mod customer_keys;
pub mod licence;
pub mod vector_search;

use std::str::FromStr;

pub use cornucopia_async::Params;
pub use deadpool_postgres::{Pool, PoolError, Transaction};
pub use licence::Licence;
pub use queries::api_keys::ApiKey;
pub use queries::audit_trail::AuditTrail;
pub use queries::categories::Category;
pub use queries::chats::Chat;
pub use queries::connections::{ApiKeyConnection, Oauth2Connection};
pub use queries::conversations::{Conversation, ConversationContextSize};
pub use queries::datasets::Dataset;
pub use queries::document_pipelines::DocumentPipeline;
pub use queries::history::History;
pub use queries::integrations::Integration;
pub use queries::invitations::{Invitation, InviteSummary};
pub use queries::models::{Model, ModelWithPrompt};
pub use queries::oauth_clients::OauthClient;
pub use queries::object_storage::ObjectStorage;
pub use queries::prompt_integrations::{PromptIntegration, PromptIntegrationWithConnection};
pub use queries::prompts::{Prompt, PromptDataset, SinglePrompt};
pub use queries::rate_limits::RateLimit;
pub use queries::teams::GetUsers as Member;
pub use queries::teams::{Team, TeamOwner};
pub use queries::users::User;
pub use tokio_postgres::types::Json;
pub use tokio_postgres::Error as TokioPostgresError;
pub use types::public::{
    AuditAccessType, AuditAction, ChatRole, ChatStatus, IntegrationType, ModelCapability,
    ModelType, Permission, PromptType, Role, TokenUsageType, Visibility,
};
pub use vector_search::{get_related_context, RelatedContext};

pub fn create_pool(database_url: &str) -> deadpool_postgres::Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();
    let manager = deadpool_postgres::Manager::new(config, tokio_postgres::NoTls);

    deadpool_postgres::Pool::builder(manager).build().unwrap()
}

include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));
