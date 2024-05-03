pub mod authz;
pub mod vector_search;

use std::str::FromStr;

pub use cornucopia_async::Params;
pub use deadpool_postgres::{Pool, PoolError, Transaction};
pub use tokio_postgres::Error as TokioPostgresError;

pub use queries::api_keys::ApiKey;
pub use queries::audit_trail::{AuditTrail, TopUser};
pub use queries::chats::Chat;
pub use queries::conversations::Conversation;
pub use queries::datasets::Dataset;
pub use queries::document_pipelines::DocumentPipeline;
pub use queries::invitations::Invitation;
pub use queries::models::Model;
pub use queries::prompts::Prompt;
pub use queries::rate_limits::RateLimit;
pub use queries::teams::GetUsers as Member;
pub use queries::teams::{Team, TeamOwner};
pub use queries::users::User;
pub use types::public::{
    AuditAccessType, AuditAction, ChatStatus, ModelType, Permission, Role, Visibility,
};
pub use vector_search::{get_related_context, RelatedContext};

pub fn create_pool(database_url: &str) -> deadpool_postgres::Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();
    let manager = deadpool_postgres::Manager::new(config, tokio_postgres::NoTls);

    deadpool_postgres::Pool::builder(manager).build().unwrap()
}

include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));
