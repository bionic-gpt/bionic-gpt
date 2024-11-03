use anyhow::Result;

use k8s_openapi::api::apps::v1::Deployment;
use kube::runtime::wait::await_condition;
use kube::runtime::wait::Condition;
use kube::Api;
use kube::Client;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// A rust wrapper around the postrgres operator CRD
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct BootstrapSpec {
    pub initdb: InitDBSpec,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct InitDBSpec {
    pub database: String,
    pub owner: String,
    pub secret: SecretSpec,
    #[serde(rename = "postInitSQL")]
    pub post_init_sql: Option<Vec<String>>,
    #[serde(rename = "postInitApplicationSQL")]
    pub post_init_application_sql: Option<Vec<String>>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SecretSpec {
    pub name: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct StorageSpec {
    pub size: String,
}

/// Corresponds to the Cluster resource
#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[kube(
    group = "postgresql.cnpg.io",
    version = "v1",
    kind = "Cluster",
    plural = "clusters",
    derive = "PartialEq",
    namespaced
)]
pub struct ClusterSpec {
    pub instances: i32,
    pub bootstrap: BootstrapSpec,
    pub storage: StorageSpec,
}

const CNPG_YAML: &str = include_str!("../config/postgres-operator.yaml");

// Install Cloud Native Postgres Operator https://cloudnative-pg.io/
pub async fn install_postgres_operator(client: &Client) -> Result<()> {
    println!("Installing Cloud Native Postgres Operator (CNPG)");
    super::apply::apply(client, CNPG_YAML, None).await?;

    fn is_deployment_available() -> impl Condition<Deployment> {
        |obj: Option<&Deployment>| {
            if let Some(deployment) = &obj {
                if let Some(status) = &deployment.status {
                    if let Some(phase) = &status.available_replicas {
                        return phase >= &1;
                    }
                }
            }
            false
        }
    }

    println!("Waiting for Cloud Native Postgres Controller Manager");
    let deploys: Api<Deployment> = Api::namespaced(client.clone(), "cnpg-system");
    let establish = await_condition(
        deploys,
        "cnpg-controller-manager",
        is_deployment_available(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(120), establish)
        .await
        .unwrap();

    Ok(())
}
