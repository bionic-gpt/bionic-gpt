use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A Bionic resource.
#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[kube(
    group = "bionic-gpt.com",
    version = "v1",
    kind = "Bionic",
    plural = "bionics",
    derive = "PartialEq",
    namespaced
)]
pub struct BionicSpec {
    pub replicas: i32,
    pub version: String,
    pub gpu: Option<bool>,
    pub saas: Option<bool>,
    pub disable_ingress: Option<bool>,
    pub pgadmin: Option<bool>,
    pub observability: Option<bool>,
    pub development: Option<bool>,
    pub testing: Option<bool>,
    pub bionic_db_disk_size: i32,
    pub keycloak_db_disk_size: i32,
    #[serde(rename = "hostname-url")]
    pub hostname_url: String,
    #[serde(rename = "hash-bionicgpt")]
    pub hash_bionicgpt: String,
    #[serde(rename = "hash-bionicgpt-rag-engine")]
    pub hash_bionicgpt_pipeline_job: String,
    #[serde(rename = "hash-bionicgpt-db-migrations")]
    pub hash_bionicgpt_db_migrations: String,
}
