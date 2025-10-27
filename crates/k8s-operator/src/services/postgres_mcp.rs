use super::deployment;
use crate::error::Error;
use crate::operator::crd::BionicSpec;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};

pub const NAME: &str = "postgres-mcp";
pub const PORT: u16 = 8080;

// Postgres MCP - exposes Postgres tooling via MCP interface
pub async fn deploy(client: Client, spec: BionicSpec, namespace: &str) -> Result<(), Error> {
    let image_name = if spec.hash_bionicgpt_pipeline_job.is_empty() {
        format!("{}:{}", super::BIONICGPT_POSTGRES_MCP_IMAGE, spec.version)
    } else {
        format!(
            "{}@{}",
            super::BIONICGPT_POSTGRES_MCP_IMAGE,
            spec.hash_bionicgpt_pipeline_job
        )
    };

    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: NAME.to_string(),
            image_name,
            replicas: spec.replicas,
            port: PORT,
            env: vec![],
            init_container: None,
            command: Some(deployment::Command {
                command: vec![],
                args: vec![],
            }),
            volume_mounts: vec![],
            volumes: vec![],
        },
        namespace,
    )
    .await?;

    Ok(())
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get(NAME).await.is_ok() {
        api.delete(NAME, &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    if api.get(NAME).await.is_ok() {
        api.delete(NAME, &DeleteParams::default()).await?;
    }

    Ok(())
}
