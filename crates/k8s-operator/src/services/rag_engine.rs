use super::deployment;
use crate::error::Error;
use crate::operator::crd::BionicSpec;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::Service;
use kube::api::DeleteParams;
use kube::{Api, Client};
use serde_json::json;

const RAG_ENGINE: &str = "bionic-rag-engine";

// The RAG Engine
pub async fn deploy(client: Client, spec: BionicSpec, namespace: &str) -> Result<(), Error> {
    let image_name = if spec.hash_bionicgpt_pipeline_job.is_empty() {
        format!("{}:{}", super::BIONICGPT_RAG_ENGINE_IMAGE, spec.version)
    } else {
        format!(
            "{}@{}",
            super::BIONICGPT_RAG_ENGINE_IMAGE,
            spec.hash_bionicgpt_pipeline_job
        )
    };

    // We used to call this something else, upgarde any existing
    // users.
    delete_old_pipeline_job(client.clone(), namespace).await?;

    deployment::deployment(
        client.clone(),
        deployment::ServiceDeployment {
            name: RAG_ENGINE.to_string(),
            image_name,
            replicas: spec.replicas,
            port: 3000,
            env: vec![
                json!({
                    "name":
                    "APP_DATABASE_URL",
                    "valueFrom": {
                        "secretKeyRef": {
                            "name": "database-urls",
                            "key": "application-url"
                        }
                    }
                }),
                json!({
                    "name":
                    "CHUNKING_ENGINE",
                    "value":
                    "http://chunking-engine:8000"
                }),
            ],
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

pub async fn delete_old_pipeline_job(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get("pipeline-job").await.is_ok() {
        api.delete("pipeline-job", &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    if api.get("pipeline-job").await.is_ok() {
        api.delete("pipeline-job", &DeleteParams::default()).await?;
    }

    Ok(())
}

pub async fn delete(client: Client, namespace: &str) -> Result<(), Error> {
    // Remove deployments
    let api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    if api.get(RAG_ENGINE).await.is_ok() {
        api.delete(RAG_ENGINE, &DeleteParams::default()).await?;
    }

    // Remove services
    let api: Api<Service> = Api::namespaced(client.clone(), namespace);
    if api.get(RAG_ENGINE).await.is_ok() {
        api.delete(RAG_ENGINE, &DeleteParams::default()).await?;
    }

    Ok(())
}
